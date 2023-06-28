
use super::data::*;
use super::function::*;

use std::collections::{HashMap};
use std::rc::Rc;

pub struct Context {
    symbol_map:HashMap<String,DataValue>,
}

// has function/variable: just check if ident in map. if it is, check the type
    // DataValue::FunctionVariable => function, else variable

// take in a dyn Function and add to map
impl Context {
    pub fn new()->Context {
        let symbol_map:HashMap<String,DataValue>=HashMap::new();
        Context {
            symbol_map
        }
    }

    pub fn add_variable(&mut self, ident:&str, value:DataValue) {
        self.symbol_map.insert(ident.to_string(), value);
    }

    pub fn add_function(&mut self, name:&str, function:Rc<dyn Function>) {
        let d=DataValue::FunctionVariable(function);
        self.symbol_map.insert(name.to_string(), d);
    }
    
    // reference is enough: we never have to mutate
    pub fn get_function(&self, name:&str)->Option<&Rc<dyn Function>> {
       self.symbol_map.get(name).and_then(|data| data.get_function())
    }

    // get a variable (non-function) - returns None if name doesn't exist or name is a function
    pub fn get_variable(&self, name:&str)->Option<&DataValue> {
        if self.get_function(name).is_some() {
            None
        } else {
            self.symbol_map.get(name)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::evaluator::builtins::*;
   #[test]
   fn context_test() {
        let fnc=Add{};
        let fnc_var=Rc::new(fnc);
        let mut ctx=Context::new();

        let num=Num(20);

        ctx.add_function("add", fnc_var);
        ctx.add_variable("x", num);

        let g=ctx.get_variable("x");
        assert_eq!(g.unwrap().get_num(), Some(20));

        let f=ctx.get_function("add");
        assert_eq!(f.unwrap().to_string(), "add".to_string());

        let mut ctx2=Context::new();

        ctx2.add_variable("y", Num(30));

        // can clone from a previous context -> maintains Rc pointers
        let add_clone=ctx.get_function("add").unwrap().clone();
        ctx2.add_function("my_add", add_clone);
        assert_eq!(ctx2.get_function("my_add").unwrap().to_string(), "add");

        let add_rc_ref=ctx.get_function("add").unwrap();
        assert_eq!(Rc::strong_count(add_rc_ref), 2); // 2: both ctx and ctx2 point to it
   }

   #[test]
   fn context_test_overwrite() {
        let fnc=Add{};
        let fnc_var=Rc::new(fnc);
        let mut ctx=Context::new();

        let num=Num(20);

        ctx.add_function("add", fnc_var);
        assert!(ctx.get_function("add").is_some());

        ctx.add_variable("add", num);
        assert!(ctx.get_function("add").is_none());

        // can overwrite again
        let fnc=Add{};
        let fnc_var=Rc::new(fnc);
        ctx.add_function("add", fnc_var);
        assert!(ctx.get_function("add").is_some());
   }
}