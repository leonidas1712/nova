use crate::parser::node::*;
use super::function::Function;

// Number, Boolean, List, String, Lambda, FunctionVariable(Box<dyn Function>)
// when we have an enum that has a reference, then a vector of enums and I clone the vector what happens

pub enum DataValue<'a> {
    Num(usize),
    Boolean(bool),
    FunctionVariable(&'a Box<dyn Function>), // we need to borrow the function from Context when doing this
    Default,
}
pub enum Arg<'a> {
    Evaluated(DataValue<'a>),
    Unevaluated(ASTNode),
    DefaultArg
}

pub use Arg::*;
pub use DataValue::*;

