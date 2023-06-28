use super::builtins::*;
use super::data::*;
use super::function::*;
use crate::message::*;

pub struct Context {
    functions: Vec<Box<dyn Function>>,
}

// has function/variable: just check if ident in map. if it is, check the type
    // DataValue::FunctionVariable => function, else variable
impl Context {
    pub fn new() -> Context {
        let add = Add;
        let sub = Sub;

        let uf = UserFunction::new("recr");
        let uf2 = UserFunction::new("recr_tail");

        // turn into macro given list of function names => Box::new...
        let functions: Vec<Box<dyn Function>> =
            vec![Box::new(Add), Box::new(Sub), Box::new(uf), Box::new(uf2)];

        Context { functions }
    }

    pub fn test(&self) -> Result<DataValue> {
        for function in self.functions.iter() {
            let args: Vec<Arg> = vec![DefaultArg];

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
        let ctx = Context::new();
        println!("test eval");
        ctx.test().unwrap();
    }

    // Question: if I have a Vec<&DataValue> then clone it is the original still usable
    // then what about hashmap with string => &DataValue
    #[test]
    fn test_clone() {
        // we can own the data because DataValue is just an enum and FunctionVar already has a reference(?)
        // cloning: can we clone the enum?
        let mut v: Vec<DataValue> = Vec::new();
        let num = Num(30);
        let add: Box<dyn Function> = Box::new(Add);
        let add_var = FunctionVariable(&add);

        if let FunctionVariable(adder) = add_var {
            adder.execute(vec![], &Context::new());
        }

        v.push(num);
        v.push(add_var);

        let v2_cloned = v.clone();

        let x = v2_cloned
            .get(1)
            .unwrap()
            .get_function()
            .unwrap()
            .execute(vec![], &Context::new());

        println!("After v2 clone");

        v.get(1)
            .unwrap()
            .get_function()
            .unwrap()
            .execute(vec![], &Context::new());
    }
}

pub fn context() {
    println!("Context");
}
