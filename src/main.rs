#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![recursion_limit = "5000"]

use nova::evaluator::data::Arg::Evaluated;
use nova::evaluator::data::DataValue::Num;
// (recr_t 6000 0)
use nova::{self, setup_context};
use nova::parser::parser::parse;
use nova::evaluator::context::*;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, Ref, RefMut};
use std::{env::args, rc::Rc};

struct FuncReturn {
    ctx:Context
}

#[derive (Clone)]
struct EvalContext {
    ctx:Rc<RefCell<Context>>
}

impl EvalContext {
    pub fn new()->EvalContext {
        EvalContext {
            ctx:Rc::new(RefCell::new(setup_context()))
        }
    }

    pub fn read(&self)->Ref<Context> {
        self.ctx.as_ref().borrow()
    }

    pub fn write(&self)->RefMut<Context> {
        self.ctx.as_ref().borrow_mut()
    }
}

// takes a Rc<Rf> ctx and reads from it then returns out another Rc<Rf<ctx>>
fn func1(ctx:EvalContext, assign:(&str,i64))->FuncReturn{
    let mut ctx2=ctx.read().clone();
    ctx2.add_variable(assign.0, Num(assign.1));
    FuncReturn { ctx: ctx2 }
}

fn main() {
   let ev=EvalContext::new();
   let vars:[(&str,i64);3]=[("x",1),("y",2),("z",3)];
   let vars2=vars.clone();

   for tup in vars.into_iter() {
        let re=func1(ev.clone(), tup.clone());
        println!("ok");

        let new_ctx=re.ctx;
        ev.write().write_context(new_ctx);

        let y=ev.read();

        let var_name=tup.0.clone();
        let var=y.get_variable(tup.0);
        let var=var.unwrap();
        println!("{}:{}", var_name, var.to_string());
   }

   for tup in vars2.into_iter() {
       let var=tup.0.clone();
       let ctx=ev.read();
       let get=ctx.get_variable(var).unwrap();
       println!("{}:{}", var, get.to_string())
   }

//    ev.write().add_variable("x", Num(23)); this is illegal because mutable ref above + immutable ref here
}

// (range 5 10) >> for_each $ puts => prints 5,6,7,..10

// (recr 1)
// (add 2 (id 1) (id 1) 3)
// (id (id 1))
// (fn (a) (succ a))
    // (add 2 (fn 1) 3)
