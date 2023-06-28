#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use nova;
use std::env::args;

fn main() {
    let args = args().into_iter();
    nova::run(args);
}
