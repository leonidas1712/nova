use crate::constants::*;
use crate::message::*;

use super::context::*;
use super::data::*;
use super::function::*;

use DataValue::*;

macro_rules! name {
    ($name:expr) => {
        String::from(format!("<function '{}'>", $name))
    };
}

macro_rules! ev {
    ($args:expr) => {
        Arg::expect_all_eval($args)?
    };
}

macro_rules! check {
    ($name:expr, $num:expr, $args:expr) => {
        if $args.len() != $num {
            let msg = format!(
                "'{}' expected {} arguments but received {}.",
                $name,
                $num,
                $args.len()
            );
            return err!(msg);
        }
    };
}

fn get_nums(args: Vec<Arg>) -> Result<Vec<NumType>> {
    let r: Result<Vec<NumType>> =
        Arg::expect_all_eval(args).and_then(|f| f.into_iter().map(|x| x.expect_num()).collect());
    return r;
}

// individual builtins: tell what to do when you have all args
// builtin struct: impl Function there, do currying logic etc
    // args: finite or inf => focus on just finite first
// state: arg names, arg idx 
pub struct Add;
impl Function for Add {
    fn execute(&self, args: Vec<Arg>, _context: &EvalContext) -> Result<DataValue> {
        let r = get_nums(args);
        let total: Result<NumType> = r.map(|v| v.into_iter().sum());
        total.map(|n| Num(n))
    }

    fn to_string(&self) -> String {
        // ADD.to_string()
        name!(ADD)
    }
}

pub struct Sub;
impl Function for Sub {
    fn execute(&self, args: Vec<Arg>, _context: &EvalContext) -> Result<DataValue> {
        get_nums(args)
            .map(|v| v.into_iter().reduce(|acc, e| acc - e))?
            .ok_or(Ex::new("Could not subtract provided expression"))
            .map(|x| Num(x))
    }

    fn to_string(&self) -> String {
        name!(SUB)
    }
}

pub struct Mult;
impl Function for Mult {
    fn execute(&self, args: Vec<Arg>, context: &EvalContext) -> Result<DataValue> {
        get_nums(args)
            .map(|v| v.into_iter().reduce(|acc, e| acc * e))?
            .ok_or(Ex::new("Could not multiply provided expression"))
            .map(|x| Num(x))
    }

    fn to_string(&self) -> String {
        name!(MULT)
    }
}

pub struct Equals;
impl Function for Equals {
    fn execute(&self, args: Vec<Arg>, context: &EvalContext) -> Result<DataValue> {
        let eval_args = ev!(args);
        check!(EQUALS, 2, eval_args);

        let left = eval_args.get(0).unwrap();
        let right = eval_args.get(1).unwrap();

        Ok(Bool(left.equals(right)))
    }

    fn to_string(&self) -> String {
        name!(EQUALS)
    }
}

pub struct Succ;
impl Function for Succ {
    fn execute(&self, args: Vec<Arg>, context: &EvalContext) -> Result<DataValue> {
        let eval_args = get_nums(args)?;
        check!(INC, 1, eval_args);

        eval_args
            .get(0)
            .map(|x| Num(x + 1))
            .ok_or(Ex::new("Couldn't add num."))
    }

    fn to_string(&self) -> String {
        name!(INC)
    }
}

pub struct Pred;
impl Function for Pred {
    fn execute(&self, args: Vec<Arg>, context: &EvalContext) -> Result<DataValue> {
        let eval_args = get_nums(args)?;
        check!(DEC, 1, eval_args);

        eval_args
            .get(0)
            .map(|x| Num(x - 1))
            .ok_or(Ex::new("Couldn't subtract num.")) // err unreachable
    }

    fn to_string(&self) -> String {
        name!(DEC)
    }
}
