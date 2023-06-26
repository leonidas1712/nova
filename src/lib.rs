pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod constants;
pub mod time;

use rustyline::{error::ReadlineError, DefaultEditor};

pub fn run(mut args: impl Iterator<Item=String>) {
    args.next();
    args.for_each(|s| println!("{}",s));
    lexer::lex();
    nova_repl();
}


pub fn nova_repl() {
    let mut rl = DefaultEditor::new().unwrap();
    
    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(inp) => {
                if inp.len()==0 {
                    continue;
                }

                println!("You typed: {}", inp);
                rl.add_history_entry(inp).unwrap();
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("See you again!");
                break;            
            }
            _ => (),
        }
    }
}
