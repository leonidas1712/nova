use super::context::*;
use super::data::*;
use super::evaluator;
use crate::message::*;
use crate::parser::parse_node::*;

// &Context: need to be able to re-use the context
pub trait Function {
    fn execute(&self, args: Vec<Arg>, context: &Context) -> Result<DataValue>;

    // default: Evaluated
    fn get_arg_type(&self) -> ArgType {
        ArgType::Evaluated
    }

    fn to_string(&self) -> String {
        "Function trait".to_string()
    }
}

#[derive(Clone)]
pub struct UserFunction {
    name: String,
    context: Context,
}

impl UserFunction {
    pub fn new(name: &str) -> UserFunction {
        UserFunction {
            name: name.to_string(),
            context: Context::new(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("User function name: {}", self.name)
    }
}

impl Function for UserFunction {
    fn execute(&self, _args: Vec<Arg>, context: &Context) -> Result<DataValue> {
        // just test by passing name
        evaluator::eval!(
            &context,
            &ASTNode::new(ParseValue::Symbol(self.name.clone()))
        )?;

        Ok(Default)
    }
}
