// ASTNode: type, value:
    // value: Option<Vec<ASTNode>> or a concrete value
// enum: NodeValue<T>
    // NodeValue::Children(Vec<ASTNode>)
    // NodeValue::Value(T)

use std::ops::Deref;

// evaluation: evaluate node => Result<T,E>
    // T: some custom wrapper struct/enum e.g NodeResult
    // then NodeResult has another enum for the different data types
        // e.g NodeResult::Number
        // NodeResult::List
use crate::message::Result;
use crate::{lexer, message::{NovaError, NovaResult}};
use crate::constants::{OPEN_TOKENS,CLOSE_TOKENS};

#[derive(Debug, Display)]
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

impl Deref for ASTNode {
    type Target = NodeValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}


// Parser
pub mod parser {
    use crate::constants::{CLOSE_EXPR, EXPR_TUP, LIST_TUP};
    use super::*;

    fn parse_list_expression(lex:&mut lexer::Lexer)->Result<ASTNode> {
        let open_token=lex.next().expect("Received empty expression");

        let mut children:Vec<ASTNode>=Vec::new();

        // loop and get child expressions
        let opt_token:Option<&str> = loop {
            match lex.peek().map(|x| x.as_str()) {
                Some(token) if CLOSE_TOKENS.contains(&token) => {
                    break Some(token)
                },
                None => break None,
                _ => ()
            }

            let res=parse_expression(lex)?;
            children.push(res.result);
        };

        // compare first and last token: should match () or []
        // if we broke out of loop without a closing token => not well formed e.g  (2
        match opt_token {
            Some(last_token) => {
                dbg!("Got last token: {}", &last_token);
                let cmp=(open_token.as_str(),last_token);
                if cmp!=EXPR_TUP && cmp!=LIST_TUP {
                    return Err(NovaError::new("Mismatched brackets."));
                };
            },
            None => return Err(NovaError::new("Expression was not well-formed."))
        };

        return Ok(NovaResult::new(ASTNode::new(NodeValue::Expression(children))));
    }

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

        let token=token_peek.unwrap().as_str();

        // if first token is ), not well formed
        if CLOSE_TOKENS.contains(&token) {
            return Err(NovaError::new("Expression is not well formed."))
        }

        // list
        if OPEN_TOKENS.contains(&token) {
            return parse_list_expression(lex);
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

    use lexer::Lexer;
    #[cfg(test)]
    #[test]
    pub fn parse_atomic_test() {

        let mut lex=&mut Lexer::new("let".to_string()).unwrap().result;
        let res=parser::parse_atomic_expression(lex).unwrap();
        if let NodeValue::Symbol(v)=&res.value {
            assert_eq!(v, "let");
        } else {
            assert!(false);
        }
    }   

    #[test]
    pub fn parse_list_expression_test() {
        let mut lex=&mut Lexer::new("(add 2 3 (add 2 3))".to_string()).unwrap().result;
        let res=parser::parse_list_expression(lex).unwrap();

        dbg!(&res.value);

        if let NodeValue::Expression(children) = &res.value {
            // first layer: add,2,3, (add 2 3)
            let v:Vec<String>=children.iter().map(|x| x.value.to_string()).collect();
            assert_eq!(v, vec!["Symbol", "Number", "Number", "Expression"]);

            let v2=&children.get(3).unwrap().value;

            // second layer: add,2,3
            if let NodeValue::Expression(cr2)=&v2 {
                let v3:Vec<String>=cr2.iter().map(|x| x.value.to_string()).collect();
                assert_eq!(v3, vec!["Symbol", "Number", "Number"]);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn parse_list_expression_test_err() {
        let mut lex=&mut Lexer::new("(add".to_string()).unwrap().result;
        let res=parser::parse_list_expression(lex).unwrap_err();
        assert!(&res.format_error().contains("not well-formed."));

        let mut lex=&mut Lexer::new("(1,2]".to_string()).unwrap().result;
        let res=parser::parse_list_expression(lex).unwrap_err();
        assert!(&res.format_error().contains("Mismatched brackets"));

    }
}


