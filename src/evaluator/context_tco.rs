use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

use std::rc::Rc;

use crate::constants::*;

use super::builtins_tco::*;
use super::data_tco::*;
use super::function_tco::*;

// wrapper around Rc<RefCell<Context>>
#[derive(Clone)]
pub struct EvalContext {
    ctx: Rc<RefCell<Context>>,
}

impl EvalContext {
    pub fn new() -> EvalContext {
        EvalContext {
            ctx: Rc::new(RefCell::new(setup_context())),
        }
    }

    pub fn new_from_context(ctx: &Context) -> EvalContext {
        let new_ctx = ctx.clone();
        EvalContext {
            ctx: Rc::new(RefCell::new(new_ctx)),
        }
    }

    // eval version
    // makes a new context with key value pairs inserted from other_ctx only if they don't exist
    pub fn merge_context(&self, other_ctx: &EvalContext) -> EvalContext {
        let mut new_ctx = self.read().clone();

        for (key, value) in &other_ctx.read().symbol_map {
            if new_ctx.get_data_value(&key).is_none() {
                new_ctx.symbol_map.insert(key.clone(), value.clone());
            }
        }
        EvalContext::new_from_context(&new_ctx)
    }

    // writes key value pairs from other_ctx into self, consuming other_ctx
    // useful for returning out from evaluate: we don't need the returned ctx after copying in
    pub fn write_context(&mut self, other_ctx: EvalContext) {
        for (key, value) in other_ctx.read().to_owned().symbol_map.into_iter() {
            self.write().add_variable(key.as_str(), value);
        }
    }

    pub fn read(&self) -> Ref<Context> {
        self.ctx.as_ref().borrow()
    }

    pub fn write(&mut self) -> RefMut<Context> {
        self.ctx.as_ref().borrow_mut()
    }

    // clone gives the same Rc ptr, we need a method for making a new copy
    pub fn copy(&self) -> EvalContext {
        let existing = self.read().clone();
        EvalContext {
            ctx: Rc::new(RefCell::new(existing)),
        }
    }

    pub fn to_string(&self) -> String {
        let mut vars: Vec<String> = vec![];
        let mut fns: Vec<String> = vec![];

        let ctx = self.read();

        for key in ctx.symbol_map.keys() {
            let var = ctx.get_function(key);

            if var.is_some() {
                let repr = format!("Function: {} => {}", key, var.unwrap().to_string());
                fns.push(repr);
                continue;
            }

            let var = ctx.get_variable(key).unwrap();
            let repr = format!("Variable: {} => {}", key, var.to_string());
            vars.push(repr);
        }

        vars.sort_by(|a, b| a.cmp(b));

        fns.sort_by(|a, b| a.cmp(b));

        let mut res = fns.join("\n");
        res.push_str("\n\n");
        res.push_str(vars.join("\n").as_str());

        res
    }
}

#[derive(Clone)]
pub struct Context {
    pub symbol_map: HashMap<String, DataValue>,
}

pub fn setup_context() -> Context {
    let mut ctx = Context::new();
    macro_rules! reg {
        ($name:expr, $struct:ident) => {
            ctx.add_function($name, Rc::new($struct {}));
        };
    }

    macro_rules! regb {
        ($fn:expr) => {
            let b = $fn();
            ctx.add_function(b.name.clone().as_str(), Rc::new(b));
        };
    }

    regb!(build_add);
    regb!(build_sub);
    regb!(build_mult);
    regb!(build_equals);
    regb!(build_succ);
    regb!(build_pred);
    regb!(build_puts);
    regb!(build_chain);

    // reg!(ADD, Add);
    // reg!(SUB, Sub);
    // reg!(MULT, Mult);
    // reg!(EQUALS, Equals);
    // reg!(INC, Succ);
    // reg!(DEC, Pred);
    // reg!(PRINT, Print);
    // reg!(CHAIN, Chain);

    ctx
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

    pub fn delete_variable(&mut self, name: &str) {
        self.symbol_map.remove(name);
    }

    pub fn add_function(&mut self, name: &str, function: Rc<dyn Function>) {
        let d = DataValue::FunctionVariable(function);
        self.symbol_map.insert(name.to_string(), d);
    }

    // for getting something either a variable or a function
    pub fn get_data_value(&self, name: &String) -> Option<&DataValue> {
        self.symbol_map.get(name)
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

    pub fn to_string(&self) -> String {
        let mut pairs: Vec<String> = vec![];
        for (key, value) in self.symbol_map.iter() {
            let m = format!("{}: {}", key.to_string(), value.to_string());
            pairs.push(m);
        }
        pairs.join("\n")
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{constants::NumType, evaluator::builtins_tco::*};

    #[test]
    fn eval_context_test() {
        // check rc strong count increases when .clone() and doesnt increase on .copy
        let mut c1 = EvalContext::new();
        c1.write().add_variable("x", Num(3));

        let mut c2 = c1.clone();
        c2.write().add_variable("y", Num(5));

        let has_y = c1.read();
        let has_y = has_y.get_variable("y");
        assert!(has_y.is_some()); // clone is Rc::clone

        let c3 = c2.clone();

        let count = Rc::strong_count(&c1.ctx);
        assert_eq!(count, 3);

        // new
        let mut c1_to_copy = EvalContext::new();
        c1_to_copy.write().add_variable("x", Num(3));

        let mut c2_copied = c1_to_copy.copy();
        c2_copied.write().add_variable("y", Num(5));

        assert!(c1_to_copy.read().get_variable("y").is_none()); // none because .copy
        let count = Rc::strong_count(&c1_to_copy.ctx);
        assert_eq!(count, 1);
    }

    #[test]
    fn context_test() {
        let fnc = build_add();
        let fnc_name = fnc.to_string();

        let fnc_var = Rc::new(fnc);
        let mut ctx = Context::new();

        let num = Num(20);

        ctx.add_function("add", fnc_var);
        ctx.add_variable("x", num);

        let g = ctx.get_variable("x");
        let exp: NumType = 20;
        assert_eq!(g.unwrap().expect_num().unwrap(), exp);

        let f = ctx.get_function("add");
        assert_eq!(f.unwrap().to_string(), fnc_name);

        let mut ctx2 = Context::new();

        ctx2.add_variable("y", Num(30));

        // can clone from a previous context -> maintains Rc pointers
        let add_clone = ctx.get_function("add").unwrap().clone();
        ctx2.add_function("my_add", add_clone);
        assert_eq!(ctx2.get_function("my_add").unwrap().to_string(), fnc_name);

        let add_rc_ref = ctx.get_function("add").unwrap();
        assert_eq!(Rc::strong_count(add_rc_ref), 2); // 2: both ctx and ctx2 point to it
    }

    #[test]
    fn context_test_overwrite() {
        let fnc = build_add();
        let fnc_var = Rc::new(fnc);
        let mut ctx = EvalContext::new();

        let num = Num(20);

        ctx.write().add_function("add", fnc_var);
        assert!(ctx.read().get_function("add").is_some());

        ctx.write().add_variable("add", num);
        assert!(ctx.read().get_function("add").is_none());

        // can overwrite again
        let fnc = build_add();
        let fnc_var = Rc::new(fnc);
        ctx.write().add_function("add", fnc_var);
        assert!(ctx.read().get_function("add").is_some());

        let mut c1 = EvalContext::new();
        c1.write().add_variable("x", Num(2));

        let mut c2 = EvalContext::new();
        c2.write().add_variable("y", Num(5));

        c1.write_context(c2);

        let c1x = c1.read().get_variable("x").unwrap().expect_num().unwrap();
        assert_eq!(c1x, 2);

        let c1y = c1.read().get_variable("y").unwrap().expect_num().unwrap();
        assert_eq!(c1y, 5);
    }

    #[test]
    fn context_test_clone() {
        let mut c = Context::new();
        c.add_variable("x", Num(2));
        c.add_variable("y", Num(3));

        let mut c2 = c.clone();
        c2.add_variable("x", Num(5));

        let x = c.get_variable("x").unwrap().expect_num().unwrap();
        assert_eq!(x, 2);

        let y = c.get_variable("y").unwrap().expect_num().unwrap();
        assert_eq!(y, 3);

        let x = c2.get_variable("x").unwrap().expect_num().unwrap();
        assert_eq!(x, 5);

        let y = c2.get_variable("y").unwrap().expect_num().unwrap();
        assert_eq!(y, 3);

        c2.add_variable("new", Num(5));
        assert_eq!(c.get_variable("new").is_none(), true);
    }

    #[test]
    pub fn context_test_merge() {
        let mut c = EvalContext::new();

        c.write().add_variable("x", Num(2));
        c.write().add_variable("y", Num(3));

        let mut c2 = c.copy();

        c2.write().add_variable("x", Num(5));
        c2.write().add_variable("y", Num(10));
        c2.write().add_variable("z", Num(10));

        let c3 = c.merge_context(&c2);
        assert_eq!(
            c3.read()
                .get_data_value(&"x".to_string())
                .unwrap()
                .expect_num()
                .unwrap(),
            2
        );
        assert_eq!(
            c3.read()
                .get_data_value(&"z".to_string())
                .unwrap()
                .expect_num()
                .unwrap(),
            10
        );
    }
}
