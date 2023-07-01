#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![recursion_limit = "5000"]

// (recr_t 6000 0)
use nova;
use std::{env::args, cell::RefCell, rc::Rc};
use crate::nova::parser::parse_node::*;

fn main() {
    let args = args().into_iter();
    nova::run(args);
}

// (recr 1)
// (add 2 (id 1) (id 1) 3)
// (id (id 1))
// (fn (a) (succ a))
    // (add 2 (fn 1) 3)
