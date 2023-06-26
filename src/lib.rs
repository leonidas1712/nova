pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod time;
pub mod repl;

pub fn run(mut args: impl Iterator<Item=String>) {
    args.next();
    args.for_each(|s| println!("{}",s));
    lexer::lex();
    repl::nova_repl();
}
