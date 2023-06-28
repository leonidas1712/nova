use super::function::Function;
use crate::parser::node::*;
use std::rc::Rc;

// Number, Boolean, List, String, Lambda, FunctionVariable(Box<dyn Function>)
// when we have an enum that has a reference, then a vector of enums and I clone the vector what happens

// do a getter for each enum type that returns an Option so we can chain map etc

// Function shouldn't get dropped until all refs in context/args are dropped -> use Rc
#[derive(Clone)]
pub enum DataValue {
    Num(usize),
    Boolean(bool),
    FunctionVariable(Rc<dyn Function>), // we need to borrow the function from Context when doing this
    Default,
}

impl DataValue  {
    pub fn to_string(&self) -> String {
        match self {
            Num(number) => number.to_string(),
            Boolean(b) => b.to_string(),
            FunctionVariable(f) => f.to_string(),
            Default => "Default Data Value".to_string(),
        }
    }

    pub fn get_num(&self) -> Option<usize> {
        match self {
            Num(num) => Some(*num),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Boolean(bool) => Some(*bool),
            _ => None,
        }
    }

    pub fn get_function(&self) -> Option<&Rc<dyn Function>> {
        match self {
            FunctionVariable(fn_ref) => Some(fn_ref),
            _ => None,
        }
    }
}

pub enum Arg<'a> {
    Evaluated(DataValue),
    Unevaluated(ASTNode),
    DefaultArg,
}

// impl<'a> Arg<'a> {
//     fn get_args_evaluated(value_iter: impl Iterator<Item=DataValue<'a>>)->impl Iterator<Item=Arg<'a>> {
//         value_iter.map(|val| Evaluated(val))
//     }
// }

pub enum ArgType {
    Evaluated,
    Unevaluated,
}

pub use Arg::*;
pub use DataValue::*;
