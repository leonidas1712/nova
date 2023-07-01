#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![recursion_limit = "5000"]

// (recr_t 6000 0)
use nova;
use nova::parser::parser::parse;
use std::{env::args, rc::Rc};

fn main() {
    let args = args().into_iter();
    nova::run(args);
}

// (range 5 10) >> for_each $ puts => prints 5,6,7,..10

// (recr 1)
// (add 2 (id 1) (id 1) 3)
// (id (id 1))
// (fn (a) (succ a))
    // (add 2 (fn 1) 3)
