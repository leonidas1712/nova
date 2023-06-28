use super::function::Function;
use crate::parser::node::ASTNode;
use std::rc::Rc;

pub const NUM:&str="Num";
pub const BOOL:&str="Bool";
pub const FNV:&str="FunctionVariable";

// Number, Boolean, List, String, Lambda, FunctionVariable(Box<dyn Function>)
// when we have an enum that has a reference, then a vector of enums and I clone the vector what happens

// do a getter for each enum type that returns an Option so we can chain map etc

// Function shouldn't get dropped until all refs in context/args are dropped -> use Rc
#[derive(Clone,Display)]
pub enum DataValue {
    Num(usize),
    Boolean(bool),
    FunctionVariable(Rc<dyn Function>), // we need to borrow the function from Context when doing this
    Default,
}

impl DataValue  {
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

    // does this transfer ownership because of Rc instead of &Rc
    // let d={Rc<..>}, then rf=&d, then *(rf.rcp), then see
    // make a DataVal with a function, do get and call, then do it again
    pub fn get_function(&self) -> Option<&Rc<dyn Function>> {
        match self {
            FunctionVariable(fn_ref) => Some(fn_ref),
            _ => None,
        }
    }
}

pub enum Arg {
    Evaluated(DataValue),
    Unevaluated(ASTNode),
    DefaultArg,
}

pub enum ArgType {
    Evaluated,
    Unevaluated,
}

pub use Arg::*;
pub use DataValue::*;

#[cfg(test)]
    pub mod tests {
        use super::DataValue::*;
        use super::super::builtins::Add;
        use std::rc::Rc;

        #[test]
        fn data_test_getters() {
            let d1=Num(20);
            let d2=Boolean(true);
            let add=Add{};
            let d3=FunctionVariable(Rc::new(add));

            dbg!(d3.to_string());

            assert_eq!(d1.get_num().unwrap(), 20);
            assert!(d2.get_num().is_none());
            assert!(d3.get_num().is_none());

            assert!(d1.get_bool().is_none());
            assert!(d2.get_bool().unwrap());
            assert!(d3.get_bool().is_none());

            assert!(d1.get_function().is_none());
            assert!(d2.get_function().is_none());
            assert!(d3.get_function().is_some());            
        }
    }
