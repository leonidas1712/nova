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
use crate::message::Result;
use crate::{lexer, message::{NovaError, NovaResult}};


#[derive(Debug)]
pub enum NodeValue {
    Symbol(String),
    Number(usize),
    Expression(Vec<ASTNode>)
}

// ASTNode
#[derive(Debug)]
pub struct ASTNode {
    value:NodeValue
}

impl ASTNode {
    fn new(value:NodeValue)->ASTNode {
        ASTNode { value }
    }
}


// Parser
pub mod parser {
    use super::*;
    // fn parse_list_expression(lex:&mut lexer::Lexer)->Result<ASTNode> {

    // }

    fn parse_atomic_expression(lex:&mut lexer::Lexer)->Result<ASTNode> {
        let token_opt=lex.next();
        if token_opt.is_none() {
           return Err(NovaError::new("Problem parsing expression."))
        }

        let token=token_opt.unwrap();
        let try_numeric=token.parse::<usize>();

        let node=match try_numeric {
            Ok(num) => {
                ASTNode::new(NodeValue::Number(num))
            },

            Err(_) => {
                ASTNode::new(NodeValue::Symbol(token))
            }
        };

        Ok(NovaResult::new(node))
    }
    
    // recursive
    fn parse_expression(lex:&mut lexer::Lexer)->Result<ASTNode>{
        let token_peek=lex.peek();
        if let None = token_peek {
            return Err(NovaError::new("Unrecognised expression."))
        }

        // Check cases in order, last is atomic expression
        
        parse_atomic_expression(lex)
    }

    pub fn parse(mut lex:lexer::Lexer)->Vec<ASTNode> {
        let mut nodes:Vec<ASTNode>=Vec::new();
        
        loop {
            if let None=lex.peek() {
                break;
            }

            let res=parse_expression(&mut lex);
            if let Ok(nr) = res {
                nodes.push(nr.result);
            }
        }
        return nodes;
    }

    #[cfg(test)]
    #[test]
    pub fn parse_atomic_test() {
        use std::ops::Deref;

        let mut lex=&mut lexer::Lexer::new("k".to_string()).unwrap().result;
        let res=parser::parse_atomic_expression(lex);
        dbg!(res.ok().unwrap());
        // assert_eq!(*res.unwrap().result, NodeValue::Number(23));
    }   
}


