use super::context::*;
use super::data::*;
use super::function::*;
use crate::message::*;

pub struct Add;
impl Function for Add {
    fn execute(&self, args: Vec<Arg>, context: &Context) -> Result<DataValue> {
        println!("Added");

        Ok(NovaResult::new(Default))
    }

    fn to_string(&self) -> String {
        "add".to_string()
    }
}

pub struct Sub;
impl Function for Sub {
    fn execute(&self, args: Vec<Arg>, context: &Context) -> Result<DataValue> {
        println!("Sub");

        Ok(NovaResult::new(Default))
    }
}
