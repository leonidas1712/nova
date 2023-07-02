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

use crate::{
    constants::*,
    evaluator::{builtins::*, context::{Context,setup_context}, 
    evaluator::evaluate,evaluator_tco::evaluate_outer

    },
};
use evaluator::context::EvalContext;
use rustyline::{error::ReadlineError, DefaultEditor};

// main function to take a string -> get a string representing output
// take a mut ctx -> in only 2 cases we need to add something:
// FnDef and (set x...) => have special types in DataValue for this
pub fn evaluate_input(inp: &str, context: &mut EvalContext) -> String {
    let res = lexer::Lexer::new(inp.to_string())
        .and_then(|lex| parser::parser::parse(lex))
        .and_then(|node| evaluate(context.clone(), node, true));

    // context.add_function(name, function)

    use crate::evaluator::data::DataValue::*;
    use evaluator::function::*;

    match res {
        Ok(val) => {
            // dbg!(&val);
            let mut string = val.to_string();
            // let mut mut_context=context.write();

            //set outer context here
            if let SetVar(data) = val {
                let ret_ctx = data.context;
                let value = data.value;
                string = value.to_string();

                context.write_context(*ret_ctx);

            } else if let SetFn(rc) = val {
                let name = rc.get_name();
                let rc2: Rc<dyn Function> = rc;
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

    let ctx = EvalContext::new();

    nova_repl(ctx);
}

pub fn nova_repl(mut context: EvalContext) {
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

                let res = evaluate_input(inp.as_str(), &mut context);
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
