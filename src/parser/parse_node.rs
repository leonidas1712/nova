use crate::constants::*;

use std::fmt::Display;
use std::ops::Deref;
use std::rc::{Rc};

// functions that return other functions but outer args no match
    // (x->y->(add x y)) (1) (2)
    // (def curr (x) (def curr2 (y) (add x y)));
        // (curr 1 2) => err

// todo: IfStmt, LetStmt, FnDef, Lambda, FunctionCall
// FunctionCall: when we have a symbol or expression which is:
// 1. nested inside another expression (more than 1 inside parent.value)
// 2. is the first element inside that expression

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Rc<ASTNode>>, // can have multiple expressions in body,
    pub global:bool
}

impl FnDef {
    pub fn set_global(&self, global:bool)->FnDef {
        FnDef {
            name:self.name.clone(),
            params:self.params.clone(),
            body:self.body.clone(),
            global
        }
    }
}

impl Display for FnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body_string: Vec<String> = self.body.iter().map(|n| n.to_string()).collect();
        let body_string = body_string.join(SPACE);

        let param_string = self.params.join(SPACE);
        let param_string = format!("{}{}{}", OPEN_EXPR, param_string, CLOSE_EXPR);

        let _st = format!(
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
    ParseExpression(Vec<Rc<ASTNode>>),
    List(Vec<Rc<ASTNode>>),
    Boolean(bool),
    IfNode(Vec<Rc<ASTNode>>),
    LetNode(Vec<Rc<ASTNode>>, bool),
    FnNode(FnDef),
}

impl ParseValue {
    pub fn get_symbol(&self) -> Option<String> {
        match self {
            Symbol(sym) => Some(sym.to_string()),
            _ => None,
        }
    }

    pub fn get_expression(&self) -> Option<Vec<Rc<ASTNode>>> {
        match self {
            ParseExpression(nodes) => Some(nodes.clone().to_vec()),
            _ => None,
        }
    }
}

pub use ParseValue::*;

// 1. compare some uniquely gen id
// 2. or compare Rc's
// ASTNode
use uuid::Uuid;
#[derive(Debug)]
pub struct ASTNode {
    pub value: ParseValue,
    // when there is a parent, we need the parent ref to be valid -> needs Rc
    // otherwise we can't do evaluation properly
    // the cycle ends when we reach root with parent=None
    // then the initial parent can get dropped and the children get dropped successively
    pub parent: Option<Rc<ASTNode>>,
    pub original: Uuid,
}

impl Clone for ASTNode {
    fn clone(&self) -> Self {
        ASTNode {
            value: self.value.clone(),
            parent: self.parent.clone(),
            original: Uuid::new_v4(),
        }
    }
}

// compare by id instead of checking values (too much time)
impl PartialEq for ASTNode {
    fn eq(&self, other: &Self) -> bool {
        self.original.eq(&other.original)
    }
}

impl Eq for ASTNode {}

//  (app (def h(x) x) 1) -> 1 (currently err)
impl ASTNode {
    pub fn new(mut value: ParseValue) -> ASTNode {
        // when clone node: the clone node should return true on equals cmp
        // issue now is just that if we do node.parent.value, that value wont be the
        // same as node -> but we only care about checking parent equality for now

        let original_ref = Uuid::new_v4();

        let original = ASTNode {
            value: value.clone(),
            parent: None,
            original: original_ref,
        };
        let original = Rc::new(original);

        match &mut value {
            ParseExpression(ref mut children)
            | List(ref mut children)
            | LetNode(ref mut children, _)
            | IfNode(ref mut children) => {
                let children = children.clone();
                let mut children: Vec<ASTNode> =
                    children.into_iter().map(|r| r.as_ref().clone()).collect();

                children
                    .iter_mut()
                    .for_each(|child| child.parent = Some(Rc::clone(&original)));

                let children: Vec<Rc<ASTNode>> = children.into_iter().map(|r| Rc::new(r)).collect();

                let new_value = match value {
                    ParseExpression(_) => ParseExpression(children),
                    IfNode(_) => IfNode(children),
                    LetNode(_, global) => LetNode(children, global),
                    List(_) => List(children),
                    _ => value, //unreachable
                };

                ASTNode {
                    value: new_value,
                    parent: None,
                    original: original_ref,
                }
            }
            _ => ASTNode {
                value,
                parent: None,
                original: original_ref,
            },
        }
    }

    pub fn copy(&self)->ASTNode {
        ASTNode {
            value:self.value.clone(),
            parent:self.parent.clone(),
            original:self.original
        }
    }

    pub fn empty() -> ASTNode {
        ASTNode::new(Symbol("Default parent".to_string()))
    }

    pub fn get_type(&self) -> String {
        self.value.to_string()
    }

    pub fn get_children(&self) -> Option<&Vec<Rc<ASTNode>>> {
        if let ParseExpression(children) | List(children) = &self.value {
            Some(&children)
        } else {
            None
        }
    }

    pub fn get_ith_child(&self, index: usize) -> Option<&Rc<ASTNode>> {
        self.get_children().and_then(|v| v.get(index))
    }
    
    pub fn to_string_with_parent(&self) -> String {
        let parent_string = match &self.parent {
            Some(prt) => prt.to_string(),
            None => String::from("None"),
        };

        let self_string = self.to_string();

        format!(
            "\n[\n\ttype:{}\n\tself:{},\n\tparent:{}\n]\n",
            self.get_type(),
            self_string,
            parent_string
        )
    }

    pub fn to_string(&self) -> String {
        match &self.value {
            Symbol(string) => string.clone(),
            Number(num) => num.to_string(),
            ParseExpression(children) => {
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

#[test]
fn parse_node_test_clone() {
    let c1 = ASTNode::new(Number(2));
    let c1_cloned = c1.clone();
    // direct clone not equal
    assert_ne!(c1, c1_cloned);

    let c1 = Rc::new(c1);
    let c1 = Some(c1);

    let c2 = c1.clone();

    let c1_unwrap = c1.unwrap();
    let c2_unwrap = c2.unwrap();
    // but clone of optional of same node is equal
    assert_eq!(c1_unwrap, c2_unwrap);

    let c1_cloned = Rc::new(c1_cloned);
    let c1_cloned = Some(c1_cloned);

    let c1_clone_unwrap = c1_cloned.unwrap();

    // direct clone wrapped in opt not equal
    assert_ne!(c1_unwrap, c1_clone_unwrap);
}
