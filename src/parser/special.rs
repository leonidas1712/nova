use crate::constants::*;

use crate::evaluator::eval_helpers::is_valid_identifier;
use crate::lexer;
use crate::message::*;
use crate::parser::parse_node::*;
use super::parser::*;

pub (super) fn parse_special(spec_type: Special, children: Vec<ASTNode>)->Result<ASTNode> {
    match spec_type {
        Special::If => return parse_if_expression(children),
        Special::Let => return parse_let_expression(children),
        Special::Fn => return parse_fn_def(children)
    }
}

pub (super) fn parse_fn_def(children: Vec<ASTNode>)->Result<ASTNode> {
    Ok(ASTNode::new(Symbol("FnDef".to_string())))
}

pub (super) fn parse_if_expression(children: Vec<ASTNode>)->Result<ASTNode> {    
    if children.len()!=4 {
        let msg=format!("'{}' expected 3 expressions but got {}.", IF_NAME, children.len());
        return err!(msg);
    }

    let mut children=children.into_iter();
    children.next();

    let cond=children.next().unwrap();
    let e1=children.next().unwrap();
    let e2=children.next().unwrap();

    let res=vec![cond,e1,e2];

    let node_val=IfNode(res);
    Ok(ASTNode::new(node_val))
}


// change to return tuple (ident, expr) since we are checking anyway
pub (super) fn parse_let_expression(children: Vec<ASTNode>)->Result<ASTNode> {
    // when parsing symbol: do parse atomic, check valid ident
    // else: parse normally
    if children.len()==1 {
        let msg=format!("'{}' received 0 expressions or symbols", LET_NAME);
        return err!(&msg);
    }

    // remove let
    let mut children=children.into_iter();
    children.next();
    
    let children=children.collect();

    Ok(ASTNode::new(LetNode(children)))
}

use lexer::*;
use super::parser::tests::*;

#[test]
pub fn parse_let_test() {
    let e1="(let x 2)";
    let e2="(let x 2 x)";
    let e3="(let x 2 y 3 (add x y))";
    let e4="(let x 2 (add x y))";
    test_parse(vec![e1,e2,e3,e4]);
}

#[test]
fn if_test() {
    let exprs=vec![
        "(if 1 2 3)",
        "(if (if 0 1 2) (add 5 6) (sub x (mul 4 5)))",
        "(if (3 4 5) (true false) false)",
    ];

    test_parse(exprs);
}