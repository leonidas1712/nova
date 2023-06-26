use rustyline::{error::ReadlineError, DefaultEditor};

pub fn nova_repl() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(inp) => {
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
