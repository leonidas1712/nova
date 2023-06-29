use crate::constants::*;
use std::fmt::Display;
use std::ops::Deref;

// todo: IfStmt, LetStmt, FnDef, Lambda, FunctionCall
// FunctionCall: when we have a symbol or expression which is:
// 1. nested inside another expression (more than 1 inside parent.value)
// 2. is the first element inside that expression

#[derive(Debug, Display)]
pub enum NodeValue {
    Symbol(String),
    Number(NumType),
    Expression(Vec<ASTNode>),
    List(Vec<ASTNode>),
    Boolean(bool),
    IfNode(Vec<ASTNode>),
    LetNode(Vec<ASTNode>)
}

pub use NodeValue::*;

// ASTNode
#[derive(Debug)]
pub struct ASTNode {
    pub value: NodeValue,
}

impl ASTNode {
    pub fn new(value: NodeValue) -> ASTNode {
        ASTNode { value }
    }

    pub fn empty() -> ASTNode {
        ASTNode::new(Symbol("empty".to_string()))
    }

    pub fn get_children(&self) -> Option<&Vec<ASTNode>> {
        if let Expression(children) | List(children) = &self.value {
            Some(&children)
        } else {
            None
        }
    }

    pub fn get_ith_child(&self, index: usize) -> Option<&ASTNode> {
        self.get_children().and_then(|v| v.get(index))
    }

    pub fn to_string(&self) -> String {
        match &self.value {
            Symbol(string) => string.clone(),
            Number(num) => num.to_string(),
            Expression(children) => {
                let v: Vec<String> = children.iter().map(|n| n.to_string()).collect();
                format!("{}{}{}", OPEN_EXPR, v.join(SPACE), CLOSE_EXPR)
            }
            List(children) => {
                let v: Vec<String> = children.iter().map(|n| n.to_string()).collect();
                format!("{}{}{}", OPEN_LIST, v.join(VAR_SEP), CLOSE_LIST)
            },
            IfNode(children) => {
                let v: Vec<String> = children.iter().map(|n| n.to_string()).collect();
                format!("{}{} {}{}", OPEN_EXPR, IF_NAME, v.join(SPACE), CLOSE_EXPR)
            },
            LetNode(children)=>{
                let v: Vec<String> = children.iter().map(|node| node.to_string()).collect();
                format!("{}{} {}{}", OPEN_EXPR, LET_NAME, v.join(SPACE), CLOSE_EXPR)
            }
            Boolean(b) => if *b { TRUE.to_string() } else { FALSE.to_string() },
        }
    }
}

impl Deref for ASTNode {
    type Target = NodeValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
