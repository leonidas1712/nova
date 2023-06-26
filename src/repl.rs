use rustyline::{error::ReadlineError, DefaultEditor};

pub fn nova_repl() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(inp) => {
                println!("You typed: {}", inp);
            }
            Err(ReadlineError::Interrupted) => {
                println!("See you again!");
                break;            
            }
            _ => (),
        }
    }
}
