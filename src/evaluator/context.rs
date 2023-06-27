use std::collections::HashMap;

use crate::parser::*;
use crate::parser::node::*;
use crate::message::*;
use crate::evaluator::evaluator;

use super::data::*;
use super::function::*;
use super::builtins::*;

pub struct Context {
    functions: Vec<Box<dyn Function>>
}

impl Context {
    pub fn new()->Context {
        let add=Add;
        let sub=Sub;

        let uf=UserFunction::new("recr");
        let uf2=UserFunction::new("recr_tail");


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