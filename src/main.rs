#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![recursion_limit = "5000"]

use nova::evaluator::data::DataValue::Num;
// (recr_t 6000 0)
use nova::{self, setup_context};
use nova::parser::parser::parse;
use nova::evaluator::context::*;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::{env::args, rc::Rc};

struct FuncReturn {
    ctx:Rc<RefCell<Context>>
}

// takes a Rc<Rf> ctx and reads from it then returns out another Rc<Rf<ctx>>
// fn func1(ctx:Rc<RefCell<Context>>)->FuncReturn{
//     let mut ctx2=ctx.as_ref().borrow().clone();
//     ctx2.add_variable("y", Num(2));
//     FuncReturn { ctx: Rc::new(RefCell::new(ctx2)) }
// }

fn main() {
    // let ctx=Context::new();
    // let ctx=RefCell::new(ctx);
    // let ctx=Rc::new(ctx);

    // func1(Rc::clone(&ctx));

    // ctx.as_ref().borrow_mut().add_variable("g", Num(3));

    // let mut c_ref=ctx.as_ref().borrow_mut();
    // let g=c_ref.get_variable("g");
    // println!("{}",g.unwrap().to_string());

    // c_ref.add_variable("g2", Num(4));
    // let g2=c_ref.get_variable("g2");

    // func1(Rc::clone(&ctx));
    // println!("{}",g2.unwrap().to_string());

    // let y=c_ref.get_variable("y");
    // println!("{}", y.is_some());

    let args = args().into_iter();
    nova::run(args);
}

// (range 5 10) >> for_each $ puts => prints 5,6,7,..10

// (recr 1)
// (add 2 (id 1) (id 1) 3)
// (id (id 1))
// (fn (a) (succ a))
    // (add 2 (fn 1) 3)
