use crate::constants::*;
use crate::message::*;

use super::context::*;
use super::data::*;
use super::function::*;

use DataValue::*;

macro_rules! name  {
    ($name:expr) => {
        String::from(format!("<function '{}'>", $name))
    };
}

fn get_nums(args: Vec<Arg>) -> Result<Vec<NumType>> {
    let r: Result<Vec<NumType>> =
        Arg::expect_all_eval(args).and_then(|f| f.into_iter().map(|x| x.expect_num()).collect());
    return r;
}

pub struct Add;
impl Function for Add {
    fn execute(&self, args: Vec<Arg>, _context: &Context) -> Result<DataValue> {
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
    fn execute(&self, args: Vec<Arg>, _context: &Context) -> Result<DataValue> {
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
    fn execute(&self, args: Vec<Arg>, context: &Context) -> Result<DataValue> {
        get_nums(args)
            .map(|v| v.into_iter().reduce(|acc, e| acc * e))?
            .ok_or(Ex::new("Could not multiply provided expression"))
            .map(|x| Num(x))
    }

    fn to_string(&self) -> String {
        name!(MULT)
    }
}
