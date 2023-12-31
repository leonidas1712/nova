use core::num;
use std::cmp::Ordering;
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
use super::params::Params;

// &Context: need to be able to re-use the context
pub trait Function {
    fn apply(&self, args: &[Arg]) -> Rc<dyn Function>;

    fn execute(&self, args: &[Arg], context: &EvalContext) -> Result<Expression>;

    fn resolve(&self, context: &EvalContext) -> Result<Expression>;

    // default: Evaluated
    fn get_arg_type(&self) -> ArgType {
        ArgType::Evaluated
    }

    // args so far for resolve
    // fn get_params(&self)->&Params;

    // num expected params - remove later
    // fn get_num_expected_params(&self) -> NumParams;

    fn to_string(&self) -> String;
}

use crate::parser::parse_node::FnDef;

// BuiltIn: name String, params:Params
// name, params, body
#[derive(Clone)]
pub struct UserFunction {
    context: EvalContext, // ctx at creation - user only
    name: String,         // b also
    params: Params,
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
        let mut stored_ctx = context.copy();

        stored_ctx.write().delete_variable(&fn_def.name);
        let params = fn_def.params.clone();
        let params = params.iter().map(|x| x.as_str()).collect();

        UserFunction {
            context: stored_ctx, // copy to get new copy that doesn't affect
            name: fn_def.name.clone(),
            params: Params::new_finite(params),
            // params:fn_def.params.clone(),
            // params_idx:0,
            body: fn_def.body.clone(), // ASTNode.clone
        }
    }
    // should be called at time of execution
    pub fn curry(&self, args: &[Arg]) -> Result<EvalContext> {
        let mut new_ctx = EvalContext::new();
        let eval_args = Arg::expect_all_eval(args)?;
        let num_args = eval_args.len();

        if self.params.expected_params().is_none() {
            return Ok(new_ctx);
        }

        // let num_expected=self.num_expected_params().unwrap();
        // let num_expected=self.params.expected_params().unwrap().len();
        // let num_expected;
        // println!("Params diff:{:?}", self.params.get_finite().unwrap().params_diff());
        let finite = self.params.get_finite().expect("Should be finite");
        let actual_params = finite.actual_params();

        // can't curry for too many
        if num_args > actual_params.len() {
            println!("gt_here");
            let msg = format!(
                "'{}' expected {} arguments but received {}.",
                self.get_name(),
                finite.params.len(),
                num_args
            );
            return err!(msg);
        }

        // add args to context using params
        // need to account for already in
        let curried_params = actual_params.clone().into_iter().take(num_args);

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

        let params: Vec<String> = self
            .params
            .expected_params()
            .unwrap_or(vec!["*args".to_string()]);

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
    fn resolve(&self, context: &EvalContext) -> Result<Expression> {
        match &self.params {
            Params::Finite(fin) => match fin.params_diff() {
                // when less return curried function
                Ordering::Less => Ok(EvaluatedExpr(FunctionVariable(Rc::new(self.clone())))),
                Ordering::Equal => self.execute(&fin.received_args, context),
                Ordering::Greater => {
                    let msg = format!(
                        "'{}' expected {} arguments but received {}.",
                        self.name,
                        fin.params.len(),
                        fin.received_args.len()
                    );
                    err!(msg)
                }
            },
            Params::Infinite(inf) => {
                if inf.received_args.len() < inf.min {
                    Ok(EvaluatedExpr(FunctionVariable(Rc::new(self.clone()))))
                } else {
                    self.execute(&inf.received_args, context)
                }
            }
        }
    }

    // apply args and return new functiom
    fn apply(&self, args: &[Arg]) -> Rc<dyn Function> {
        // let new_idx=self.params_idx+args.len();
        // let new_ctx=self.curry(args)?;

        let new_body: Vec<Rc<ASTNode>> = self
            .body
            .iter()
            .map(|node| node.as_ref().clone())
            .map(|node| Rc::new(node))
            .collect();

        let new_fn = UserFunction {
            context: self.context.clone(),
            name: self.name.clone(),
            params: self.params.apply(args),
            body: new_body,
        };

        Rc::new(new_fn)
    }

    fn execute(&self, args: &[Arg], outer_ctx: &EvalContext) -> Result<Expression> {
        let num_args = args.len();

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

    fn to_string(&self) -> String {
        self.to_string()
    }
}

use crate::Lexer;
#[test]
fn test_curry() {
    let func = "(def fn (a b c) (add a b c))";
    let mut lx = lex!(func);
    let p = parse(&mut lx).expect("Should parse fn def");
    let ctx = EvalContext::new();

    let ev = evaluate_outer(ctx, p, true).expect("Should evaluate");

    let func = "(fn 1 2)";
    let mut lx = lex!(func);
    let p = parse(&mut lx).expect("Should parse fn def");
}
