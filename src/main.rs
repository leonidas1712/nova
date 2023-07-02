#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![recursion_limit = "5000"]

use nova::evaluator::data::Arg::Evaluated;
use nova::evaluator::data::DataValue::Num;
// (recr_t 6000 0)
use nova::evaluator::context::*;
use nova::parser::parser::parse;
use nova::{self};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell, RefMut};
use std::{env::args, rc::Rc};

fn main() {
    nova::run(args());
}

// (range 5 10) >> for_each $ puts => prints 5,6,7,..10

// (recr 1)
// (add 2 (id 1) (id 1) 3)
// (id (id 1))
// (fn (a) (succ a))
// (add 2 (fn 1) 3)
