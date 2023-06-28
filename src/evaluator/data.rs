use crate::parser::node::*;
use super::function::Function;

// Number, Boolean, List, String, Lambda, FunctionVariable(Box<dyn Function>)
// when we have an enum that has a reference, then a vector of enums and I clone the vector what happens

// do a getter for each enum type that returns an Option so we can chain map etc


#[derive(Clone)]
pub enum DataValue<'a> {
    Num(usize),
    Boolean(bool),
    FunctionVariable(&'a Box<dyn Function>), // we need to borrow the function from Context when doing this
    Default,
}

impl<'a> DataValue<'a> {
    pub fn get_num(&self)->Option<usize> {
        match self {
            Num(num) => Some(*num),
            _ => None
        }
    }

    pub fn get_bool(&self)->Option<bool> {
        match self {
            Boolean(bool) => Some(*bool),
            _ => None
        }
    }

    pub fn get_function(&self)->Option<&'a Box<dyn Function>> {
        match self {
            FunctionVariable(fn_ref) => Some(*fn_ref),
            _ => None
        }
    }
}

pub enum Arg<'a> {
    Evaluated(DataValue<'a>),
    Unevaluated(ASTNode),
    DefaultArg
}

pub enum ArgType {
    Evaluated,
    Unevaluated
}

pub use Arg::*;
pub use DataValue::*;

