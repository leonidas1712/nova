use super::data::*;
use super::function::*;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Context {
    symbol_map: HashMap<String, DataValue>,
}

// has function/variable: just check if ident in map. if it is, check the type
// DataValue::FunctionVariable => function, else variable

// take in a dyn Function and add to map
impl Context {
    pub fn new() -> Context {
        let symbol_map: HashMap<String, DataValue> = HashMap::new();
        Context { symbol_map }
    }

    pub fn add_variable(&mut self, ident: &str, value: DataValue) {
        self.symbol_map.insert(ident.to_string(), value);
    }

    pub fn add_function(&mut self, name: &str, function: Rc<dyn Function>) {
        let d = DataValue::FunctionVariable(function);
        self.symbol_map.insert(name.to_string(), d);
    }

    // reference is enough: we never have to mutate
    pub fn get_function(&self, name: &str) -> Option<&Rc<dyn Function>> {
        self.symbol_map
            .get(name)
            .and_then(|data| data.expect_function().ok())
    }

    // get a variable (non-function) - returns None if name doesn't exist or name is a function
    pub fn get_variable(&self, name: &str) -> Option<&DataValue> {
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
    use crate::{constants::NumType, evaluator::builtins::*};
    #[test]
    fn context_test() {
        let fnc = Add {};
        let fnc_var = Rc::new(fnc);
        let mut ctx = Context::new();

        let num = Num(20);

        ctx.add_function("add", fnc_var);
        ctx.add_variable("x", num);

        let g = ctx.get_variable("x");
        let exp: NumType = 20;
        assert_eq!(g.unwrap().expect_num().unwrap(), exp);

        let f = ctx.get_function("add");
        assert_eq!(f.unwrap().to_string(), "add".to_string());

        let mut ctx2 = Context::new();

        ctx2.add_variable("y", Num(30));

        // can clone from a previous context -> maintains Rc pointers
        let add_clone = ctx.get_function("add").unwrap().clone();
        ctx2.add_function("my_add", add_clone);
        assert_eq!(ctx2.get_function("my_add").unwrap().to_string(), "add");

        let add_rc_ref = ctx.get_function("add").unwrap();
        assert_eq!(Rc::strong_count(add_rc_ref), 2); // 2: both ctx and ctx2 point to it
    }

    #[test]
    fn context_test_overwrite() {
        let fnc = Add {};
        let fnc_var = Rc::new(fnc);
        let mut ctx = Context::new();

        let num = Num(20);

        ctx.add_function("add", fnc_var);
        assert!(ctx.get_function("add").is_some());

        ctx.add_variable("add", num);
        assert!(ctx.get_function("add").is_none());

        // can overwrite again
        let fnc = Add {};
        let fnc_var = Rc::new(fnc);
        ctx.add_function("add", fnc_var);
        assert!(ctx.get_function("add").is_some());
    }

    #[test]
    fn context_test_clone() {
        let mut c = Context::new();
        c.add_variable("x", Num(2));
        c.add_variable("y", Num(3));

        let mut c2=c.clone();
        c2.add_variable("x", Num(5));

        let x=c.get_variable("x").unwrap().expect_num().unwrap();
        assert_eq!(x,2);

        let y=c.get_variable("y").unwrap().expect_num().unwrap(); 
        assert_eq!(y,3);

        let x=c2.get_variable("x").unwrap().expect_num().unwrap();
        assert_eq!(x,5);

        let y=c2.get_variable("y").unwrap().expect_num().unwrap(); 
        assert_eq!(y,3);

        c2.add_variable("new", Num(5));
        assert_eq!(c.get_variable("new").is_none(), true);
        
    }
}
