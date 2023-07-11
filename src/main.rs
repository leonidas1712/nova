use nova::{self};
use std::env::args;

fn main() {
    nova::run(args());
}

// (range 5 10) >> for_each $ puts => prints 5,6,7,..10

// (recr 1)
// (add 2 (id 1) (id 1) 3)
// (id (id 1))
// (fn (a) (succ a))
// (add 2 (fn 1) 3)
