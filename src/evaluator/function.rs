use std::rc::Rc;

use super::context::*;
use super::data::*;
use super::evaluator;
use crate::constants::CLOSE_EXPR;
use crate::constants::OPEN_EXPR;
use crate::constants::SPACE;
use crate::constants::VAR_SEP;
use crate::message::*;
use crate::parser::parse_node::*;

// &Context: need to be able to re-use the context
pub trait Function {
    fn execute(&self, args: Vec<Arg>, context: &Context) -> Result<DataValue>;

    // default: Evaluated
    fn get_arg_type(&self) -> ArgType {
        ArgType::Evaluated
    }

    fn to_string(&self) -> String;
}

use crate::parser::parse_node::FnDef;

// name, params, body
// #[derive(Clone)]
pub struct UserFunction {
    context: Context,
    name: String,
    params: Vec<String>,
    body: Vec<ASTNode>,
}

struct Test<'a> {
    context:Rc<&'a Context>
}
// clone fn_def because it could have come from a closure: the original function still needs it
// same reason for context: to impl closure we need to capture ctx at time of creation
impl UserFunction {
    fn ret_node(&self, args:Vec<Arg>, outer_ctx:&Context)->Test {
        let y = self.body.get(0);
        let x=y.unwrap();
        let z = x;
        println!("{}", x.to_string());

        let new_ctx=self.curry(args).unwrap();
        
        Test { context: Rc::new(&self.context) }
    }

    pub fn new(context: &Context, fn_def: &FnDef) -> UserFunction {
        UserFunction {
            context: context.clone(),
            name: fn_def.name.clone(),
            params: fn_def.params.clone(),
            body: fn_def.body.clone(), // ASTNode.clone
        }
    }

    pub fn curry(&self, args: Vec<Arg>) -> Result<Context> {
        let mut new_ctx = Context::new();
        let eval_args = Arg::expect_all_eval(args)?;

        if eval_args.len() != self.params.len() {
            let msg = format!(
                "'{}' expected {} arguments but received {}.",
                self.get_name(),
                self.params.len(),
                eval_args.len()
            );
            return err!(msg);
        }

        // add args to context using params
        let zipped = self.params.clone().into_iter().zip(eval_args.into_iter());

        zipped.for_each(|tup| {
            new_ctx.add_variable(tup.0.as_str(), tup.1);
        });

        let new_ctx = new_ctx.merge_context(&self.context);

        Ok(new_ctx)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn to_string(&self) -> String {
        let name = &self.name;
        let params = &self.params;
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
    fn execute(&self, args: Vec<Arg>, outer_ctx: &Context) -> Result<DataValue> {
        // first clone + add arguments using params and args

        let strings: Vec<String> = args.iter().map(|x| x.to_string()).collect();
        let strings = strings.join(" ");

        let eval_ctx = self.curry(args)?;

        // then merge outer_ctx
        // args > inner_ctx > outer_ctx

        let eval_ctx = eval_ctx.merge_context(outer_ctx);

        let fn_node = self.body.get(0).unwrap(); // currently on first part

        println!("Fn_node:{}", fn_node);
        let res = evaluator::eval!(&eval_ctx, fn_node)?;

        return Ok(res);
    }

    fn to_string(&self) -> String {
        self.to_string()
    }
}
