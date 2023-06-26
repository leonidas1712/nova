use crate::lexer;
pub fn parse() {
    lexer::lex();
    println!("parse");
}
