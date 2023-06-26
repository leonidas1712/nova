// ASTNode: type, value:
    // value: Option<Vec<ASTNode>> or a concrete value
// enum: NodeValue<T>
    // NodeValue::Children(Vec<ASTNode>)
    // NodeValue::Value(T)

// evaluation: evaluate node => Result<T,E>
    // T: some custom wrapper struct/enum e.g NodeResult
    // then NodeResult has another enum for the different data types
        // e.g NodeResult::Number
        // NodeResult::List

pub struct ASTNode {
    value:NodeValue
}

pub enum NodeValue {
    Symbol(String),
    Number(usize),
    Expression(Vec<ASTNode>)
}

pub mod parser {
    use crate::lexer;
    use super::*;
    
    // recursive
    pub fn parse_expression(token:String) {

    }

    pub fn parse(mut lex:lexer::Lexer) {
        let mut nodes:Vec<ASTNode>=Vec::new();

        // while let Some(token)=lex.next() {
        //     let res=parse_expression(token);

        // }

    }
}

