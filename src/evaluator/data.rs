use crate::parser::node::*;

pub enum DataValue {
    Default,
    Num(usize)
}
pub enum Arg {
    Evaluated(DataValue),
    Unevaluated(ASTNode),
    DefaultArg
}

pub use Arg::*;
pub use DataValue::*;