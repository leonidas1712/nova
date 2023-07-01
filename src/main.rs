#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![recursion_limit = "5000"]

// (recr_t 6000 0)
use nova;
use nova::parser::parser::parse;
use std::{env::args};

use crate::nova::parser::parse_node::*;
use crate::nova::lex;
use crate::Expression;
use std::ptr;

fn main() {
    use crate::nova::lexer::Lexer;
    let l=lex!("(add 2 3)");
    let p=parse(l).unwrap();

    println!("{}", p.to_string_with_parent());

    match &p.value {
        Expression(children) => {
            println!("Node itself: {}", p.to_string_with_parent());
            let c1=children.get(0).unwrap();
            println!("First child:{}", c1.to_string_with_parent());
            
            let c1_parent=c1.parent.clone().unwrap();
            let c1_parent=c1_parent.as_ref();

            let p_ref=&p;
            println!("eq:{}", ptr::eq(c1_parent, p_ref));

        },
        _ => {

        }
    }

    let args = args().into_iter();
    nova::run(args);
}

// (range 5 10) >> for_each $ puts => prints 5,6,7,..10

// (recr 1)
// (add 2 (id 1) (id 1) 3)
// (id (id 1))
// (fn (a) (succ a))
    // (add 2 (fn 1) 3)
