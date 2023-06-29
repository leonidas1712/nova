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
#[derive(Clone)]
pub struct UserFunction {
    context: Context,
    name:String,
    params:Vec<String>,
    body: Vec<ASTNode>

}

// clone fn_def because it could have come from a closure: the original function still needs it
// same reason for context: to impl closure we need to capture ctx at time of creation
impl UserFunction {
    pub fn new(context:&Context, fn_def:&FnDef) -> UserFunction {
        UserFunction {
            context: context.clone(),
            name:fn_def.name.clone(),
            params: fn_def.params.clone(),
            body: fn_def.body.clone()
        }
    }

    pub fn get_name(&self)->String {
        self.name.clone()
    }

    fn to_string(&self) -> String {
        let name=&self.name;
        let params=&self.params;
        let body=&self.body;

        let params=params.join(VAR_SEP);
        let body_string:Vec<String>=body.iter().map(|n| n.to_string()).collect();
        let body_string=body_string.join(SPACE);

        format!("{}{}{}{} => {}", name,OPEN_EXPR,params,CLOSE_EXPR, body_string)
    }
}

impl Function for UserFunction {
    fn execute(&self, args: Vec<Arg>, outer_ctx: &Context) -> Result<DataValue> {
        // just test by passing name
        let eval_ctx=self.context.merge_context(outer_ctx);
        let fn_node=self.body.get(0).unwrap();
        
        evaluator::eval!(
            &eval_ctx,
            fn_node
        )?;

        Ok(Default)
    }

    fn to_string(&self) -> String {
        self.to_string()
    }
}
