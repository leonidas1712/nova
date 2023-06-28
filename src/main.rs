use nova;
use std::env::args;
pub mod macros;

pub (crate) use macros::*;

fn main() {
    let args = args().into_iter();
    nova::run(args);
}
