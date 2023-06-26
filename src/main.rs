use nova;
use std::{env::args};

fn main() {
    let args=args().into_iter();
    nova::run(args);
}
