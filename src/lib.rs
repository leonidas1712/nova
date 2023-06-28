#![allow(dead_code)]
#![allow(unused_variables)]
extern crate strum;
#[macro_use]
extern crate strum_macros;
pub mod constants;
pub mod evaluator;
pub mod lexer;
pub mod message;
pub mod parser;
pub mod time;
pub mod macros;

pub (crate) use evaluator::*;

use std::rc::Rc;

use crate::{
    constants::*, 
    evaluator::{context::Context,builtins::*, evaluator::evaluate},
    macros::*
};
use rustyline::{error::ReadlineError, DefaultEditor};

// register builtins here
pub fn setup_context()->Context {
    let mut ctx=Context::new();

    macro_rules! reg {
        ($name:literal, $struct:ident) => {
            ctx.add_function($name, Rc::new($struct{}));
        };
    }

    reg!("add",Add);
    reg!("sub",Sub);

    ctx
}

// main function to take a string -> get a string representing output
    // take a mut ctx -> in only 2 cases we need to add something:
        // FnDef and (set x...) => have special types in DataValue for this
pub fn evaluate_input(inp:String, context:&mut Context)->String {
    let res = lexer::Lexer::new(inp)
        .and_then(|lex| parser::parser::parse(lex))
        .and_then(|node| evaluate(&context, &node));

    // context.add_function(name, function)

    match res {
        Ok(val) => val.to_string(),
        Err(err) => err.format_error()
    }
}

// setup context by making the map of functions and pass it into Context::new, then pass it to nova_repl
// this is how we can seed Context with map of refs to functions
pub fn run(mut args: impl Iterator<Item = String>) {
    args.next();
    args.for_each(|s| println!("{}", s));

    let ctx=setup_context();

    nova_repl(ctx);
}

pub fn nova_repl(mut context:Context) {
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
                    rl.clear_screen();
                    continue;
                }

                rl.add_history_entry(inp.clone().trim()).unwrap();

                let res=evaluate_input(inp, &mut context);
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
