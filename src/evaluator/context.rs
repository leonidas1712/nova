use std::collections::HashMap;

use crate::parser::*;
use crate::parser::node::*;
use crate::message::*;
use crate::evaluator::evaluator;

use super::data::*;
use super::function::*;
use super::builtins::*;

// references to functions: don't want to clone everything everytime we copy a ctx and pass to a new expression
    // just copy the refs
    // same for variables/identifiers: just copy
    // currying: always make a new function, don't need to mutate the old one (in fact we shouldn't mutate)
        // currying new context: also make a new context, but with new identifier added
        // copy all the old references into new ctx
    // identifiers: string => &DataValue

// why separate maps: because to check if something is a function dont want to search the entire namespace
    // and also we want same namespace for functions and variables, otherwise i could just have a set of function names

// functions: map (String => &Box<dyn Function>)
// identifiers: map

// idea:
    // variables: map (String => &DataValue)
        // include FunctionVariables
    // function_names: Set(String)
        // maintain invariant for shared namespace(?) e.g x=2, x=function, get x => should be a function
    
// maintain a pointer to the previous context
    // copy: pass the pointer down

// Context problem: how to copy contexts from one expression to a child expression
    // e.g first ctx: y=4, map=some function. second ctx to eval child expr: defines y=5, but should have access to map
        // child contexts should be able to overwrite but only within themselves not in the parents
    // 1. maintain pointers to parents
        // expensive: if we recurse down 1000 levels we need to search up to 1000 levels for a variable
    // 2. clone all the data each time
        // the most flexible solution, but would be good to not have to copy data like lists and functions
            // e.g a list with 10000 elements in context 1, copied to context 2 -> expensive

// Function evaluation: needs to store context at time of creation, then arguments
    // needs to clone parent context: if we use a reference any changes would be reflected => can't have closure
    // but then if Context needs to copy by cloning -> Context needs to clone 
        // but Ctx has DataValue -> does it need to deepcopy or just copying enums?

// Function: has a inner Context (cloned, owns)
    // Context: has DataValue, if it has a function it has only a reference to the function
            
// we dont want to deepcopy everything, but copying references is ok
    // clone: make a new hashmap, copy the strings, then copy the references to data (normal copy)
    // but if the map is of names to references, then when we add something it won't live long enough
        // since it will be dropped when that function exits
    // if the map is of names to owned data -> then we need to deepcopy

// do we ever need to mutate functions => No
    // builtin: defined once, currying -> copy
    // user: defined once, currying -> copy
pub struct Context {
    functions: Vec<Box<dyn Function>>
}

impl Context {
    pub fn new()->Context {
        let add=Add;
        let sub=Sub;

        let uf=UserFunction::new("recr");
        let uf2=UserFunction::new("recr_tail");


        // turn into macro given list of function names => Box::new...
        let functions:Vec<Box<dyn Function>> = vec![
            Box::new(Add),Box::new(Sub), Box::new(uf), Box::new(uf2)
        ];

        Context {
            functions
        }
    }

    pub fn test(&self)->Result<DataValue>{
        for function in self.functions.iter() {
            let args:Vec<Arg>=vec![DefaultArg];

            function.execute(args, self)?;
        }

        Ok(NovaResult::new(DataValue::Default))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_context() {
        let ctx=Context::new();
        println!("test eval");
        ctx.test().unwrap();
    }

    // Question: if I have a Vec<&DataValue> then clone it is the original still usable
        // then what about hashmap with string => &DataValue
    #[test]
    fn test_clone() {
        // we can own the data because DataValue is just an enum and FunctionVar already has a reference(?)
            // cloning: can we clone the enum?
        let mut v:Vec<DataValue>=Vec::new();
        let num=Num(30);
        let add:Box<dyn Function>=Box::new(Add);
        let add_var=FunctionVariable(&add);

        if let FunctionVariable(adder) = add_var {
            adder.execute(vec![], &Context::new());
        }

        v.push(num);
        v.push(add_var);

        let v2_cloned=v.clone();

        match v2_cloned.get(1).unwrap() {
            FunctionVariable(adder) => {
                adder.execute(vec![], &Context::new());
            },

            _ => println!("??")
        }

        match v.get(1).unwrap() {
            FunctionVariable(adder) => {
                println!("From v after clone");
                adder.execute(vec![], &Context::new());
            },

            _ => println!("??")
        }

    }
}

pub fn context() {
    println!("Context");
}