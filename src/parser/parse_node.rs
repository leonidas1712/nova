use crate::constants::*;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// todo: IfStmt, LetStmt, FnDef, Lambda, FunctionCall
// FunctionCall: when we have a symbol or expression which is:
// 1. nested inside another expression (more than 1 inside parent.value)
// 2. is the first element inside that expression

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<ASTNode>, // can have multiple expressions in body
}

impl Display for FnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body_string: Vec<String> = self.body.iter().map(|n| n.to_string()).collect();
        let body_string = body_string.join(SPACE);

        let param_string = self.params.join(SPACE);
        let param_string = format!("{}{}{}", OPEN_EXPR, param_string, CLOSE_EXPR);

        let st = format!(
            "{} {} {} {} {} {}",
            OPEN_EXPR,
            FN_NAME,
            self.name,
            self.params.join(SPACE),
            body_string,
            CLOSE_EXPR
        );
        write!(
            f,
            "{}{} {} {} {}{}",
            OPEN_EXPR, FN_NAME, self.name, param_string, body_string, CLOSE_EXPR
        )
    }
}

#[derive(Debug, Display, Clone)]
pub enum ParseValue {
    Symbol(String),
    Number(NumType),
    Expression(Vec<ASTNode>),
    List(Vec<ASTNode>),
    Boolean(bool),
    IfNode(Vec<ASTNode>),
    LetNode(Vec<ASTNode>, bool),
    FnNode(FnDef),
}

impl ParseValue {
    pub fn get_symbol(&self) -> Option<String> {
        match self {
            Symbol(sym) => Some(sym.to_string()),
            _ => None,
        }
    }

    pub fn get_expression(&self) -> Option<Vec<ASTNode>> {
        match self {
            Expression(nodes) => Some(nodes.clone().to_vec()),
            _ => None,
        }
    }
}

pub use ParseValue::*;


// 1. compare some uniquely gen id
// 2. or compare Rc's
// ASTNode
#[derive(Debug,Clone)]
pub struct ASTNode {
    pub value: ParseValue,
    // when there is a parent, we need the parent ref to be valid -> needs Rc
        // otherwise we can't do evaluation properly
    // the cycle ends when we reach root with parent=None
        // then the initial parent can get dropped and the children get dropped successively
    pub parent: Option<Rc<ASTNode>>
}

// node.clone: the cloned should be considered same as this node

impl ASTNode {
    pub fn new(mut value: ParseValue) -> ASTNode {
        // when clone node: the clone node should return true on equals cmp
        let original=ASTNode {
            value:value.clone(),
            parent:None
        };
        let original=Rc::new(original);

        match &mut value {
            Expression(ref mut children) | 
            List(ref mut children)       |
            LetNode(ref mut children, _) |
            IfNode(ref mut children)
            => {
                children.iter_mut().for_each(|child| {
                    child.parent=Some(Rc::clone(&original))
                }
                );

                ASTNode {
                    value:value,
                    parent:None
                }
            },
            _ => {
                 ASTNode {
                    value,
                    parent: None,
                }
            }
        }

        // let original=ASTNode::empty();
        // let original=Rc::new(original);

        // ASTNode {
        //     value,
        //     parent: Rc::downgrade(&original)
        // }



        // works:
            // ASTNode {
            //     value,
            //     parent: Weak::new(),
            // }
    }

    pub fn empty() -> ASTNode {
        ASTNode::new(Symbol("Default parent".to_string()))
    }

    pub fn get_type(&self) -> String {
        self.value.to_string()
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

    // // sets parent of children
    // pub fn set_parents(parent: Rc<ASTNode>, children: &mut Vec<ASTNode>) {
    //     for child in children.iter_mut() {
    //         child.parent = Rc::downgrade(&parent);
    //     }
    // }

    pub fn to_string_with_parent(&self)->String {
        let parent_string = match &self.parent {
            Some(prt) => {
                prt.to_string()
            },
            None=>{
                String::from("None")
            }
        };

        let self_string=self.to_string();
        
        format!("\n[\n\ttype:{}\n\tself:{},\n\tparent:{}\n]\n",self.get_type(), self_string,parent_string)
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
            }
            IfNode(children) => {
                let v: Vec<String> = children.iter().map(|n| n.to_string()).collect();
                format!("{}{} {}{}", OPEN_EXPR, IF_NAME, v.join(SPACE), CLOSE_EXPR)
            }
            LetNode(children, _) => {
                let v: Vec<String> = children.iter().map(|node| node.to_string()).collect();
                format!("{}{} {}{}", OPEN_EXPR, LET_NAME, v.join(SPACE), CLOSE_EXPR)
            }
            FnNode(fn_def) => fn_def.to_string(),
            Boolean(b) => {
                if *b {
                    TRUE.to_string()
                } else {
                    FALSE.to_string()
                }
            }
        }
    }
}

impl Deref for ASTNode {
    type Target = ParseValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
