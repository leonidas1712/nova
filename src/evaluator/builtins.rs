use crate::message::*;
use crate::parser::node::*;

use super::context::*;
use super::evaluator;
use super::data::*;
use super::function::*;


pub struct Add;
impl Function for Add {
    fn execute(&self, args: Vec<Arg>, context:&Context)->Result<DataValue>{
        println!("Added");

        Ok(NovaResult::new(Default))
    }   
}

pub struct Sub;
impl Function for Sub {
    fn execute(&self, args: Vec<Arg>, context:&Context)->Result<DataValue> {
        println!("Sub");
        
        Ok(NovaResult::new(Default))
    }
}