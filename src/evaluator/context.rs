use super::builtins::*;
use super::data::*;
use super::function::*;
use crate::message::*;
use std::collections::{HashMap,HashSet};
use std::rc::Rc;

pub struct Context {
    symbol_map:HashMap<String,DataValue>,
}

// has function/variable: just check if ident in map. if it is, check the type
    // DataValue::FunctionVariable => function, else variable



// take in a dyn Function and add to map
impl Context {
    pub fn new_empty()->Context {
        let symbol_map:HashMap<String,DataValue>=HashMap::new();
        Context {
            symbol_map
        }
    }

    // iterate and get function names
    pub fn new(symbol_map:HashMap<String, DataValue>) -> Context {
        Context {
            symbol_map
        }
    }

    // add dyn Function 
        // 'a: means the string and function box refs must live at least as long as Context
        // if we did 'b generic instead: means we could accept a ref with unbounded lifetime
            // in this case that wouold not be valid, but if we are just say processing the refs and not storing
            // we could use 'b
    pub fn add_function(&mut self, name:&str, function:Rc<dyn Function>) {
        let d=DataValue::FunctionVariable(function);
        self.symbol_map.insert(name.to_string(), d);
    }
    
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
    // #[test]
    // fn test_context() {
    //     let ctx = Context::new(vec![]);
    //     println!("test eval");
    //     ctx.test().unwrap();
    // }

    // Question: if I have a Vec<&DataValue> then clone it is the original still usable
    // then what about hashmap with string => &DataValue
    // #[test]
    // fn test_clone() {
    //     // we can own the data because DataValue is just an enum and FunctionVar already has a reference(?)
    //     // cloning: can we clone the enum?
    //     let mut v: Vec<DataValue> = Vec::new();
    //     let num = Num(30);
    //     let add: Box<dyn Function> = Box::new(Add);
    //     let add_var = FunctionVariable(&add);

    //     if let FunctionVariable(adder) = add_var {
    //         adder.execute(vec![], &Context::new(vec![]));
    //     }

    //     v.push(num);
    //     v.push(add_var);

    //     let v2_cloned = v.clone();

    //     let x = v2_cloned
    //         .get(1)
    //         .unwrap()
    //         .get_function()
    //         .unwrap()
    //         .execute(vec![], &Context::new(vec![]));

    //     println!("After v2 clone");

    //     v.get(1)
    //         .unwrap()
    //         .get_function()
    //         .unwrap()
    //         .execute(vec![], &Context::new(vec![]));
    // }
}

pub fn context() {
    println!("Context");
}
