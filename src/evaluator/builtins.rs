use crate::message::*;
use crate::constants::*;

use super::context::*;
use super::data::*;
use super::function::*;
use super::data::*;

use DataValue::*;

// expect on data values inside args
// pub fn expect_args(args:&Vec<Arg>, )
    // args.get(0).
pub struct Add;
impl Function for Add {
    fn execute(&self, args: Vec<Arg>, context: &Context) -> Result<DataValue> {
        let r:Result<Vec<NumType>>=Arg::expect_all_eval(args)
        .and_then(|f| f.into_iter().map(|x| x.expect_num()).collect());
        
        let total:Result<NumType>=r.map(|v| v.into_iter().sum());
        total.map(|n| Num(n))
    }

    fn to_string(&self) -> String {
        ADD.to_string()
    }
}

pub struct Sub;
impl Function for Sub {
    fn execute(&self, args: Vec<Arg>, context: &Context) -> Result<DataValue> {

        Ok(Default)
    }

    fn to_string(&self) -> String {
        SUB.to_string()
    }
}
