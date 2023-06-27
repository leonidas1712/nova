use crate::parser::*;
use crate::parser::node::*;
use std::collections::HashMap;
use crate::message::*;
use crate::evaluator::evaluator;
use super::data::*;

trait Function {
    fn execute(&self, args: Vec<Arg>, context:&Context)->Result<DataValue>;
}

struct Add;
impl Function for Add {
    fn execute(&self, args: Vec<Arg>, context:&Context)->Result<DataValue>{
        println!("Added");

        Ok(NovaResult::new(Default))
    }   
}

struct Sub;
impl Function for Sub {
    fn execute(&self, args: Vec<Arg>, context:&Context)->Result<DataValue> {
        println!("Sub");
        
        Ok(NovaResult::new(Default))
    }
}

struct UserFunction {
    name:String
}

impl Function for UserFunction {
    fn execute(&self, args: Vec<Arg>, context:&Context)->Result<DataValue> {
        println!("User function: {}", &self.name);
        
        // just test by passing name
        evaluator::evaluate(&context, &ASTNode::new(NodeValue::Symbol(self.name.clone())), true)?;

        Ok(NovaResult::new(Default))
    }
}

pub struct Context {
    functions: Vec<Box<dyn Function>>
}

impl Context {
    pub fn new()->Context {
        let add=Add;
        let sub=Sub;
        
        let uf=UserFunction { name: "recr".to_string() };
        let uf2=UserFunction { name: "recr_tail".to_string() };


        // turn into macro given list of function names => Box::new...
        let functions:Vec<Box<dyn Function>> = vec![
            Box::new(Add),Box::new(Sub), Box::new(uf), Box::new(uf2)
        ];

        Context {
            functions
        }
    }

    pub fn test(&self)->Result<DataValue>{
        for function in self.functions.iter() {
            let args:Vec<Arg>=vec![DefaultArg];

            function.execute(args, self)?;
        }

        Ok(NovaResult::new(DataValue::Default))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_context() {
        let ctx=Context::new();
        println!("test eval");
        ctx.test().unwrap();
    }
}

pub fn context() {
    println!("Context");
}