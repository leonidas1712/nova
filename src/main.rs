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
