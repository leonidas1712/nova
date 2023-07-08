use core::num;
use std::rc::Rc;
use std::vec;

use crate::constants::CLOSE_EXPR;
use crate::constants::OPEN_EXPR;
use crate::constants::SPACE;
use crate::constants::VAR_SEP;
use crate::evaluate_all;
use crate::evaluate_one_node;
use crate::lex;
use crate::message::*;
use crate::parser::parse_node::*;
use crate::parser::parser::parse;

use super::context_tco::*;
use super::data_tco::*;
use super::evaluator_tco::*;

// &Context: need to be able to re-use the context
pub trait Function {
    fn execute(&self, args: &[Arg], context: &EvalContext) -> Result<Expression>;

    // default: Evaluated
    fn get_arg_type(&self) -> ArgType {
        ArgType::Evaluated
    }

    fn get_num_args(&self) -> NumArgs;
    // fn get_params(&self)->Params;

    fn to_string(&self) -> String;
}

// represents params for a function
    // finite: n, params body, params_idx. n = number of params (calculated by params.len and idx)
    // inf: min

#[derive (Clone)]
pub struct FiniteParams {
    pub params:Vec<String>,
    pub params_idx:usize,
    pub received_params:Vec<Arg>
}

impl FiniteParams {
    pub fn new(params:Vec<String>)->FiniteParams {
        FiniteParams { params, params_idx: 0, received_params: vec![] }
    }

    pub fn curry(&self,args:&[Arg])->FiniteParams {
        let mut new_params=self.received_params.clone();
        new_params.extend_from_slice(args);

        FiniteParams { 
            params: self.params.clone(), 
            params_idx: self.params_idx+args.len(), 
            received_params: new_params
        }
    }

    pub fn expected_params(&self)->Vec<String> {
        self.params.iter()
            .skip(self.params_idx)
            .map(|x| x.clone())
            .collect()
    }

    pub fn num_expected_params(&self)->usize {
        self.expected_params().len()
    }

    // resolve the curried parameters into a context
    pub fn resolve(&self)-> Result<impl Iterator<Item = (String,Arg)>>{
         // can't curry for too many
         let num_args=self.received_params.len();

         if num_args > self.num_expected_params() {
            let msg = format!(
                "expected {} arguments but received {}.",
                self.num_expected_params(),
                num_args
            );
            return err!(msg);
        } else {
            let new_ctx=EvalContext::new();

            let curried_params=self.expected_params()
            .clone()
            .into_iter()
            .take(num_args);

            // (param name, arg)
            let zipped = curried_params.zip(self.received_params.clone().into_iter());

            Ok(zipped)
        }
    }
}


#[derive (Clone)]
pub struct InfiniteParams {
    pub received_params:Vec<Arg>,
    pub min:usize
}

impl InfiniteParams {
    pub fn new(min:usize)->InfiniteParams {
        InfiniteParams {
            received_params:vec![],
            min
        }
    }

    pub fn curry(&self,args:&[Arg])->InfiniteParams {
        let mut new_params=self.received_params.clone();
        new_params.extend_from_slice(args);

        InfiniteParams { received_params: new_params, min: self.min }
    }
}


pub enum Params {
    Finite(FiniteParams),
    Infinite(InfiniteParams)
}

// params: just stores Arg
    // finite: String->Arg
    // inf: Vec<Arg>
// curry: add to table/array
// resolve: return a Result<EvalContext> with the params added
    // err for not enough/too many
// use:
    // Evaluated: 

impl Params {
    pub fn new_finite(params:Vec<String>)->Params {
        Params::Finite(
            FiniteParams::new(params)
        )
    }

    pub fn new_infinite(min:usize)->Params {
        Params::Infinite(
            InfiniteParams::new(min)
        )
    }

    pub fn expected_params(&self)->Option<Vec<String>> {
        match &self {
            Params::Finite(finite) => {
                let exp:Vec<String>=finite.params.iter()
                .skip(finite.params_idx)
                .map(|x| x.clone())
                .collect();
                Some(exp)
            },
            Params::Infinite(_) => None
        }
    }

    pub fn curry(&self, args:&[Arg]) {
    }

    // pub fn curry(&self, args: &[Arg]) -> Result<EvalContext> {
    //     let mut new_ctx = EvalContext::new();
    //     let eval_args = Arg::expect_all_eval(args)?;
    //     let num_args=eval_args.len();

    //     // can't curry for too many
    //     if num_args > self.num_expected_params() {
    //         let msg = format!(
    //             "'{}' expected {} arguments but received {}.",
    //             self.get_name(),
    //             self.num_expected_params(),
    //             num_args
    //         );
    //         return err!(msg);
    //     }

    //     // add args to context using params
    //         // need to account for already in
    //     let curried_params=self.expected_params()
    //         .clone()
    //         .into_iter()
    //         .take(num_args);

    //     let zipped = curried_params.zip(eval_args.into_iter());

    //     zipped.for_each(|tup| {
    //         new_ctx.write().add_variable(tup.0.as_str(), tup.1);
    //     });

    //     let new_ctx = new_ctx.merge_context(&self.context);

    //     Ok(new_ctx)
    // }

}

use crate::parser::parse_node::FnDef;


// BuiltIn: name String, params:Params
// name, params, body
#[derive(Clone)]
pub struct UserFunction {
    context: EvalContext, // ctx at creation - user only
    name: String, // b also
    params: Vec<String>, // builtin also
    params_idx:usize, // b also
    body: Vec<Rc<ASTNode>>, // user only
}
// clone fn_def because it could have come from a closure: the original function still needs it
// same reason for context: to impl closure we need to capture ctx at time of creation

// context: inner context at time of creation
// params: stores context from currying of args
// curry: curry inner params, just pass along prev ctx
// at time of execution: merge there

// inf args: curry by adding to internal Vec<Arg>
// time of exec: return curried function with the additional args
    // 
impl UserFunction {
    pub fn new(context: &EvalContext, fn_def: &FnDef) -> UserFunction {
        let mut stored_ctx=context.copy();

        stored_ctx.write().delete_variable(&fn_def.name);

        UserFunction {
            context: stored_ctx, // copy to get new copy that doesn't affect
            name: fn_def.name.clone(),
            params: fn_def.params.clone(),
            params_idx:0,
            body: fn_def.body.clone(), // ASTNode.clone
        }
    }

    pub fn expected_params(&self)->Vec<String> {
        self.params.iter()
            .skip(self.params_idx)
            .map(|x| x.clone())
            .collect()
    }

    pub fn num_expected_params(&self)->usize {
        // self.params.len()-self.params_idx
        self.expected_params().len()
    }

    // create curried function given new eval context and idx
    // body needs to be new nodes (?)
    pub fn curried_function(&self, args: &[Arg])->Result<UserFunction> {
        let new_idx=self.params_idx+args.len();
        let new_ctx=self.curry(args)?;

        let new_body:Vec<Rc<ASTNode>>=self.body
            .iter()
            .map(|node| node.as_ref().clone())
            .map(|node| Rc::new(node))
            .collect();

        Ok(UserFunction {
            context:new_ctx, // can remove this and use passed in
            name:self.name.clone(),
            params:self.params.clone(),
            params_idx:new_idx,
            body:new_body
        })
    }

    pub fn curry(&self, args: &[Arg]) -> Result<EvalContext> {
        let mut new_ctx = EvalContext::new();
        let eval_args = Arg::expect_all_eval(args)?;
        let num_args=eval_args.len();

        // can't curry for too many
        if num_args > self.num_expected_params() {
            let msg = format!(
                "'{}' expected {} arguments but received {}.",
                self.get_name(),
                self.num_expected_params(),
                num_args
            );
            return err!(msg);
        }

        // add args to context using params
            // need to account for already in
        let curried_params=self.expected_params()
            .clone()
            .into_iter()
            .take(num_args);

        let zipped = curried_params.zip(eval_args.into_iter());

        zipped.for_each(|tup| {
            new_ctx.write().add_variable(tup.0.as_str(), tup.1);
        });

        let new_ctx = new_ctx.merge_context(&self.context);

        Ok(new_ctx)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn to_string(&self) -> String {
        let name = &self.name;

        let params:Vec<String> = self.params.iter()
            .skip(self.params.len() - self.num_expected_params())
            .map(|x| x.clone())
            .collect();

        let body = &self.body;

        let params = params.join(VAR_SEP);
        let body_string: Vec<String> = body.iter().map(|n| n.to_string()).collect();
        let body_string = body_string.join(SPACE);

        format!(
            "{}{}{}{} => {}",
            name, OPEN_EXPR, params, CLOSE_EXPR, body_string
        )
    }
}

impl Function for UserFunction {
    fn execute(&self, args:&[Arg], outer_ctx: &EvalContext) -> Result<Expression> {
        let num_args=args.len();

        // return curried function if args less
        if num_args < self.num_expected_params() {
            let func=self.curried_function(args)?;
            let d=Rc::new(func);
            return Ok(
                EvaluatedExpr(
                    FunctionVariable(d)
                )
            );
        }

        // first clone + add arguments using params and args
        let eval_ctx = self.curry(args)?;

        // then merge outer_ctx
        // args > inner_ctx > outer_ctx

        let eval_ctx = eval_ctx.merge_context(&outer_ctx);
        let fn_node = self.body.get(0).unwrap(); // currently only on first part

        // IMPORTANT:node is CLONED so the clone compares unequal because id changed
        let cloned = fn_node.as_ref().clone();
      
        let res = DeferredExpression {
            ctx: eval_ctx.clone(),
            body: Rc::new(cloned),
        };

        let res = DeferredExpr(res);

        return Ok(res);
    }

    fn get_num_args(&self) -> NumArgs {
        // can change later to support *args
        Finite(self.num_expected_params())
    }
    fn to_string(&self) -> String {
        self.to_string()
    }
}

use crate::Lexer;
#[test]
fn test_curry() {
    let func="(def fn (a b c) (add a b c))";
    let mut lx=lex!(func);
    let p=parse(&mut lx).expect("Should parse fn def");
    let ctx=EvalContext::new();

    let ev=evaluate_outer(ctx,p,true)
        .expect("Should evaluate");
    
    let func="(fn 1 2)";
    let mut lx=lex!(func);
    let p=parse(&mut lx).expect("Should parse fn def");

}