use nova::{parser::parser, run};
use std::{env::args};

fn main() {
    let args=args().into_iter();
    nova::run(args);
    parse();
}
