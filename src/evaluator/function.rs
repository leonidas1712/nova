use super::context::*;
use super::data::*;
use crate::message::*;
use super::evaluator;
use crate::parser::node::*;

// &Context: need to be able to re-use the context
pub trait Function {
    fn execute(&self, args: Vec<Arg>, context:&Context)->Result<DataValue>;

    // default: Evaluated
    fn get_arg_type(&self)->ArgType {
        ArgType::Evaluated
    }

    fn to_string(&self)->String {
        "Function trait".to_string()
    }
}

pub struct UserFunction {
    name:String
}

impl UserFunction {
    pub fn new(name:&str)->UserFunction {
        UserFunction {
            name: name.to_string()
        }
    }

    pub fn to_string(&self)->String {
        format!("User function name: {}", self.name)
    }
}

impl Function for UserFunction {
    fn execute(&self, args: Vec<Arg>, context:&Context)->Result<DataValue> {

        // just test by passing name
        evaluator::evaluate(&context, &ASTNode::new(NodeValue::Symbol(self.name.clone())))?;

        Ok(NovaResult::new(Default))
    }
}