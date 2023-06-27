use super::context::*;
use super::data::*;
use crate::message::*;
use super::evaluator;
use crate::parser::node::*;

pub trait Function {
    fn execute(&self, args: Vec<Arg>, context:&Context)->Result<DataValue>;
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
        println!("User function: {}", &self.name);
        
        // just test by passing name
        evaluator::evaluate(&context, &ASTNode::new(NodeValue::Symbol(self.name.clone())), true)?;

        Ok(NovaResult::new(Default))
    }
}