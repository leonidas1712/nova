#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
extern crate strum;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate strum_macros;


pub mod constants;
pub mod evaluator;
pub mod lexer;
pub mod macros;
pub mod message;
pub mod parser;
pub mod time;
pub mod file;

use std::rc::Rc;
use std::vec;


use crate::{
    constants::*,
};

use file::import_file;
use lexer::Lexer;
use parser::parser::{parse,parse_all};
use parser::parse_node::ASTNode;
use evaluator::evaluator_tco::*;
use evaluator::data_tco::*;
use evaluator::function_tco::*;
use evaluator::context_tco::*;
use rustyline::{error::ReadlineError, DefaultEditor};
use message::*;

pub fn evaluate_one_node(node:Rc<ASTNode>, context: &mut evaluator::context_tco::EvalContext)->Result<String> {
    let res= evaluate_outer(context.clone(), node, true)?;

    let mut string = res.to_string();

    //set outer context here
    if let SetVar(data) = res {
        let ret_ctx = data.context;
        let value = data.value;
        string = value.to_string();

        context.write_context(*ret_ctx);

    } else if let SetFn(rc) = res {
        let name = rc.get_name();
        let rc2: Rc<dyn Function> = rc;
        context.write().add_function(&name, rc2);
    }
    // end set outer ctx
    Ok(string)
}

// result for just one node
pub fn evaluate_input_result(node:Rc<ASTNode>, context: &mut evaluator::context_tco::EvalContext)->String {
    let res=evaluate_one_node(node, context);
    match res {
        Ok(data) => data.to_string(),
        Err(err) => err.format_error()
    }
}

pub fn evaluate_input_tco(inp: &str, context: &mut EvalContext) -> String {
    let parse_result=Lexer::new(inp.to_string())
    .and_then(|mut lex| parser::parser::parse(&mut lex));

    match parse_result {
        Ok(node) => evaluate_input_result(node, context),
        Err(err) => err.format_error()
    }
}

pub fn evaluate_all(inp: &str, context: &mut EvalContext)->Result<Vec<String>> {
    let lexed=lex!(inp);
    let parse_nodes=parse_all(lexed)?;
    let mut results:Vec<String>=vec![];

    for node in parse_nodes {
        let res=evaluate_one_node(node, context)?;
        results.push(res);
    }

    Ok(results)
}


// setup context by making the map of functions and pass it into Context::new, then pass it to nova_repl
// this is how we can seed Context with map of refs to functions
pub fn run(mut args: impl Iterator<Item = String>) {
    args.next();
    args.for_each(|s| println!("{}", s));

    let mut ctx=evaluator::context_tco::EvalContext::new();

    let imp=import_file("stl.txt", &mut ctx);

    if let Err(err) = imp {
        println!("Import error:{}", err.format_error());
    }

    nova_repl_tco(ctx); 

    // use crate::time::{bench,time_comp};
    // bench(50); // 0.0905372397 for (recr 10000)
    // time_comp(65537);
}
pub fn nova_repl_tco(mut context: evaluator::context_tco::EvalContext) {
    let mut rl = DefaultEditor::new().unwrap();

    println!();
    println!("Welcome to Nova: a highly expressive, dynamically typed functional programming language.\nType an expression to get started.\n");

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

                let results=evaluate_all(&inp, &mut context);
            
                match results {
                    Ok(strings) => {
                        for string in strings {
                            println!("{}", string);
                        }
                    },
                    Err(err) => println!("{}", err.format_error())
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