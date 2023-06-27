use std::fmt::Display;
use std::ops::Deref;
use crate::constants::*;

#[derive(Debug, Display)]
pub enum NodeValue {
    Symbol(String),
    Number(usize),
    Expression(Vec<ASTNode>),
    List(Vec<ASTNode>)
}

pub use NodeValue::*;

// ASTNode
#[derive(Debug)]
pub struct ASTNode {
    pub value:NodeValue
}

impl ASTNode {
    pub fn new(value:NodeValue)->ASTNode {
        ASTNode { value }
    }
    
    pub fn empty()->ASTNode {
        ASTNode::new(Symbol("empty".to_string()))
    }

    pub fn get_children(&self)->Option<&Vec<ASTNode>> {
        if let Expression(children) | List(children) = &self.value {
            Some(&children)
        } else {
            None
        }
    }

    pub fn get_ith_child(&self, index:usize)->Option<&ASTNode> {
        self.get_children()
        .and_then(|v| v.get(index))
    }
    
    pub fn to_string(&self)->String {
        match &self.value {
            Symbol(string) => string.clone(),
            Number(num) => num.to_string(),
            Expression(children) => {
                let v:Vec<String>=children.iter().map(|n| n.to_string()).collect();
                format!("{}{}{}",OPEN_EXPR,v.join(SPACE),CLOSE_EXPR)
            },
            List(children) => {
                let v:Vec<String>=children.iter().map(|n| n.to_string()).collect();
                format!("{}{}{}",OPEN_LIST,v.join(VAR_SEP),CLOSE_LIST)
            }
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
