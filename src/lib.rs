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

use std::{collections::HashMap};
use crate::{
    constants::*, 
    evaluator::{context::Context,evaluator::evaluate,data::*, builtins::*, function::*}
};
use rustyline::{error::ReadlineError, DefaultEditor};


fn create_function<F: Function + 'static>(f: F) -> Box<dyn Function> {
    Box::new(f)
}

// register builtins here
// pub fn setup_context()->Context<'static> {
//     use DataValue::FunctionVariable;

//     let symbol_map:HashMap<&str,DataValue> = HashMap::new();

//     let add=create_function(Add{});
//     // let sub=create_function(Sub{});

//     let mut sym_map:HashMap<&str,DataValue>=HashMap::new();
//     sym_map.insert(ADD, FunctionVariable(&add));

//     let ctx=Context::new(sym_map);
//     ctx
// }

// setup context by making the map of functions and pass it into Context::new, then pass it to nova_repl
// this is how we can seed Context with map of refs to functions
pub fn run(mut args: impl Iterator<Item = String>) {
    args.next();
    args.for_each(|s| println!("{}", s));

    // use DataValue::FunctionVariable;

    // let symbol_map:HashMap<&str,DataValue> = HashMap::new();

    // let add=create_function(Add{});
    // // let sub=create_function(Sub{});

    // let mut sym_map:HashMap<&str,DataValue>=HashMap::new();
    // sym_map.insert(ADD, FunctionVariable(&add));

    // let ctx=Context::new(sym_map);
    
    // evaluator::evaluator::evaluate();
    // nova_repl(ctx);
}

pub fn nova_repl(context:Context) {
    use lexer::Lexer;
    use parser::parser;
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

                rl.add_history_entry(inp.clone().trim()).unwrap();

                // pass lexer to parser
                let res = Lexer::new(inp).and_then(|lex| parser::parse(lex.result));

                match res {
                    Ok(nr) => {
                        let node = &nr.result;
                        println!("Node: {}", &node);

                        // let ctx = context::Context::new();
                        // let mut evaluated = evaluate(&ctx, &node).unwrap();
                        // evaluated.add_messages(&nr);

                        // println!("Evaluated: {}", evaluated.result.to_string());
                    }
                    Err(ne) => {
                        println!("{}", ne.format_error());
                    }
                }
            }

            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("See you again!");
                break;
            }
            _ => (),
        }
    }
}
