#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![recursion_limit = "5000"]
extern crate strum;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate strum_macros;

use std::time::{Duration, Instant};
pub mod constants;
pub mod evaluator;
pub mod lexer;
pub mod macros;
pub mod message;
pub mod parser;
pub mod time;

use std::rc::Rc;

use crate::evaluator::evaluator_tco::print_max_depth_tco;
use crate::{
    constants::*,
    evaluator::{
        evaluator_tco::evaluate_outer,
    },
};
use rustyline::{error::ReadlineError, DefaultEditor};

pub fn evaluate_input_tco(inp: &str, context: &mut evaluator::context_tco::EvalContext) -> String {
    let res = lexer::Lexer::new(inp.to_string())
        .and_then(|lex| parser::parser::parse(lex))
        .and_then(|node| evaluator::evaluator_tco::evaluate_outer(context.clone(), node, true));


    match res {
        Ok(val) => {
            // dbg!(&val);
            let mut string = val.to_string();

            //set outer context here
            if let evaluator::data_tco::SetVar(data) = val {
                let ret_ctx = data.context;
                let value = data.value;
                string = value.to_string();

                context.write_context(*ret_ctx);
            } else if let evaluator::data_tco::SetFn(rc) = val {
                let name = rc.get_name();
                let rc2: Rc<dyn evaluator::function_tco::Function> = rc;
                context.write().add_function(&name, rc2);
            }
            // end set outer ctx
            string
        }
        Err(err) => err.format_error(),
    }
}


// setup context by making the map of functions and pass it into Context::new, then pass it to nova_repl
// this is how we can seed Context with map of refs to functions
pub fn run(mut args: impl Iterator<Item = String>) {
    args.next();
    args.for_each(|s| println!("{}", s));

    let ctx=evaluator::context_tco::EvalContext::new();
    nova_repl_tco(ctx); 

    // use crate::time::{bench,time_comp};
    // bench(50); // 0.0905372397 for (recr 10000)
    // time_comp(65537);
}
pub fn nova_repl_tco(mut context: evaluator::context_tco::EvalContext) {
    let mut rl = DefaultEditor::new().unwrap();

    println!();
    println!("Welcome to Nova, a highly expressive, dynamically typed functional programming language.\nType an expression to get started.\n");

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(inp) => {
                let inp = inp.trim().to_string();
                if inp.len() == 0 {
                    continue;
                }

                if QUIT_STRINGS.contains(&inp.as_str()) {
                    println!("See you again!");
                    break;
                }

                if ["cl", "clear"].contains(&inp.as_str()) {
                    let _ = rl.clear_screen();
                    continue;
                }

                rl.add_history_entry(inp.clone().trim()).unwrap();

                let res = evaluate_input_tco(inp.as_str(), &mut context);
                println!("{res}");
            }

            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("See you again!");
                break;
            }
            _ => (),
        }
    }
}