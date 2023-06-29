#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![recursion_limit = "5000"]

// (recr_t 6000 0)
use nova;
use std::env::args;
use crate::nova::time::bench;

fn main() {
    let args = args().into_iter();
    nova::run(args);
}
