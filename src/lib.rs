#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]
extern crate strum;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate strum_macros;

pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod utils;

use std::rc::Rc;
use std::vec;

use crate::{utils::constants::*, utils::file::*};

pub use utils::constants;
pub use utils::message;

pub use utils::file::{import_file, save_file, STL_FILE, USER_FILE};

use evaluator::context_tco::*;
use evaluator::data_tco::*;
use evaluator::evaluator_tco::*;
use evaluator::function_tco::*;
use lexer::{split_input, Lexer};
use message::*;
use parser::parse_node::ASTNode;
use parser::parser::{parse, parse_all};
use rustyline::{error::ReadlineError, DefaultEditor};

pub fn evaluate_one_node(
    node: Rc<ASTNode>,
    context: &mut evaluator::context_tco::EvalContext,
) -> Result<String> {
    let res = evaluate_outer(context.clone(), node, true)?;

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
pub fn evaluate_input_result(
    node: Rc<ASTNode>,
    context: &mut evaluator::context_tco::EvalContext,
) -> String {
    let res = evaluate_one_node(node, context);
    match res {
        Ok(data) => data.to_string(),
        Err(err) => err.format_error(),
    }
}

pub fn evaluate_input_tco(inp: &str, context: &mut EvalContext) -> String {
    let parse_result =
        Lexer::new(inp.to_string()).and_then(|mut lex| parser::parser::parse(&mut lex));

    match parse_result {
        Ok(node) => evaluate_input_result(node, context),
        Err(err) => err.format_error(),
    }
}

pub fn evaluate_all(inp: &str, context: &mut EvalContext) -> Result<Vec<String>> {
    let lexed = lex!(inp);
    let parse_nodes = parse_all(lexed)?;
    let mut results: Vec<String> = vec![];

    for node in parse_nodes {
        let res = evaluate_one_node(node, context)?;
        results.push(res);
    }

    Ok(results)
}

// :import, :del, :list, :save(?)
pub fn process_command(command: &str, ctx: &mut EvalContext) -> Result<()> {
    let words = split_input(command);

    if words.is_empty() {
        return err!("Empty command.");
    }

    let command = words.get(0).unwrap().as_str();
    let mut args = words.iter();
    args.next();

    match command {
        "list" => {
            println!("{}", ctx.to_string());
        }
        "del" => {
            if words.len() == 1 {
                return err!("No variables given to delete.");
            }

            let vars = args.clone();

            let vars: Vec<&String> = vars.collect();

            for var in vars.iter() {
                if BUILTINS.contains(&var.as_str()) {
                    return errf!("Can't delete builtin identifier '{}'", var);
                }

                if let None = ctx.read().get_data_value(var) {
                    return errf!("Identifier '{}' is not defined.", var);
                }
            }

            let mut ctx_write = ctx.write();
            for var in vars {
                ctx_write.delete_variable(var);
                println!("Deleted identifier:{}", var);
            }
        }
        "import" | "run" => {
            if words.len() == 1 {
                return err!("No files given to import.");
            }

            let files = args.clone();

            for file in files {
                import_file(file, ctx)?;
            }
        }
        _ => {
            println!("Unknown command: '{}'", command);
        }
    }

    Ok(())
}

pub fn nova_repl_tco(mut context: EvalContext) -> EvalContext {
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

                if inp.starts_with(CMD_PREFIX) {
                    if let Err(err) = process_command(&inp[1..], &mut context) {
                        println!("{}", err.format_error());
                    }
                    continue;
                }

                let results = evaluate_all(&inp, &mut context);

                match results {
                    Ok(strings) => {
                        for string in strings {
                            if string.len() == 0 {
                                continue;
                            }
                            println!("{}", string);
                        }
                    }
                    Err(err) => println!("{}", err.format_error()),
                }
            }

            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("See you again!");
                break;
            }
            _ => (),
        }
    }

    context
}

// setup context by making the map of functions and pass it into Context::new, then pass it to nova_repl
// this is how we can seed Context with map of refs to functions

// append new functions to end of user file

// -r: import stl
pub fn run(mut args: impl Iterator<Item = String>) {
    args.next();
    let args: Vec<String> = args.map(|x| x.to_string()).collect();
    let args = args.join("");

    let mut ctx = evaluator::context_tco::EvalContext::new();

    if args.contains('r') {
        let imp = import_file(STL_FILE, &mut ctx);

        if let Err(err) = imp {
            println!("Import error - {}", err.format_error());
        }
    }

    let final_ctx = nova_repl_tco(ctx);

    if let Err(err) = save_file(USER_FILE, final_ctx) {
        println!("Couldn't save functions to file: {}", USER_FILE);
        println!("Error:{}", err.to_string());
    }

    use crate::utils::time::{bench, time_comp};
    // bench(20); // 0.0905372397 for (recr 10000)
    // time_comp(65537);
}

pub fn run_bench() {
    use crate::utils::time::{bench, time_comp, RECR};
    bench(50, RECR, "(recr 1996)");
}