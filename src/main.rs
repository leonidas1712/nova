pub mod lexer;
pub mod parser;
pub mod evaluator;

fn main() {
    evaluator::evaluate();
    println!("Hello, world!");
}
