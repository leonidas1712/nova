use nova;
use std::env;

fn main() {
    let args=env::args().into_iter();
    nova::run(args);
}
