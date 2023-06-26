#![allow(dead_code)]
#![allow(unused_variables)]

pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod constants;
pub mod time;
pub mod message;

use rustyline::{error::ReadlineError, DefaultEditor};

pub fn run(mut args: impl Iterator<Item=String>) {
    args.next();
    args.for_each(|s| println!("{}",s));
    nova_repl();
}


// todo: make this return a Result then add Result to Lexer::new
    // and use unwrap for parser+lexer
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
                if inp.len()==0 {
                    continue;
                }
                
                rl.add_history_entry(inp.clone().trim()).unwrap();

                // pass lexer to parser
                let lex=Lexer::new(inp).unwrap().result;
                parser::parse(lex);
            },
            
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("See you again!");
                break;            
            }
            _ => (),
        }
    }
}
