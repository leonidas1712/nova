#![allow(dead_code)]
#![allow(unused_variables)]
extern crate strum;
#[macro_use] extern crate strum_macros;

pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod constants;
pub mod time;
pub mod message;

use rustyline::{error::ReadlineError, DefaultEditor};

use crate::constants::QUIT_STRINGS;

pub fn run(mut args: impl Iterator<Item=String>) {
    args.next();
    args.for_each(|s| println!("{}",s));
    nova_repl();
}


pub fn nova_repl() {
    use lexer::Lexer;
    use parser::parser;
    let mut rl = DefaultEditor::new().unwrap();
    
    println!();
    println!("Welcome to Nova, a highly expressive, dynamically typed functional programming language.\nType an expression to get started.\n");
    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(inp) => {
                let inp=inp.trim().to_string();
                if inp.len()==0 {
                    continue;
                }

                if QUIT_STRINGS.contains(&inp.as_str()) {
                    println!("See you again!");
                    break;     
                }
                
                rl.add_history_entry(inp.clone().trim()).unwrap();

                // pass lexer to parser
                let res=Lexer::new(inp).and_then(|lex| parser::parse(lex.result));     

                match res {
                    Ok(nr) => {
                        let node=nr.result;
                        println!("Node: {}", node);

                        nr.messages.iter().for_each(|msg| println!("Message: {}", msg))
                    },
                    Err(ne) => {
                        println!("{}", ne.format_error());
                    }
                }
            },
            
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("See you again!");
                break;            
            }
            _ => (),
        }
    }
}
