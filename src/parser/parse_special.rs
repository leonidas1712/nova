use std::rc::Rc;

use crate::constants::*;
use crate::evaluator::eval_helpers_tco::is_valid_identifier;
use crate::{lexer, lex};
use crate::message::*;
use crate::parser::parse_node::*;

use super::parser::*;
use super::parser::tests::test_parse;

pub(super) fn parse_special(
    spec_type: Special,
    children: Vec<Rc<ASTNode>>,
    global: bool,
) -> Result<Rc<ASTNode>> {
    match spec_type {
        Special::If => return parse_if_expression(children),
        Special::Let => return parse_let_expression(children, global),
        Special::Fn => return parse_fn_def(children, global),
    }
}

// create FnNode
// def, name, args, body
pub(super) fn parse_fn_def(children: Vec<Rc<ASTNode>>, _global: bool) -> Result<Rc<ASTNode>> {
    if children.len() < 4 {
        return err!(
            "Function definitions should have at least 3 parts: a name, parameters and a body."
        );
    }

    let mut children = children.into_iter();
    children.next(); // move past def

    let name = children.next().unwrap().get_symbol();

    if name.is_none() {
        return err!("Function name should be a symbol.");
    }

    let name = name.unwrap();
    is_valid_identifier(name.as_str())?;

    // should be inside expression or just a symbol (flattened)
    let nxt_node = children.next().unwrap();
    let mut param_nodes: Vec<Rc<ASTNode>> = vec![];

    let get_symbol = nxt_node.get_symbol();
    let get_expr = nxt_node.get_expression();

    if get_symbol.is_some() {
        param_nodes = vec![nxt_node];
    } else if get_expr.is_some() {
        param_nodes = get_expr.unwrap();
    } else {
        let msg = format!(
            "Parameters for '{}' should be a symbol or in in an expression.",
            name
        );
        return err!(msg);
    }

    // get param names from nodes
    let all_symbols: Option<Vec<String>> = param_nodes.iter().map(|p| p.get_symbol()).collect();
    if all_symbols.is_none() {
        return err!("Function parameters should contain only symbols");
    }

    // check that all names are valid idents
    let all_symbols = all_symbols.unwrap();
    let all_ok: Result<Vec<String>> = all_symbols.iter().map(|s| is_valid_identifier(s)).collect();

    let params = all_ok?;

    // end of err handling
    let rest: Vec<Rc<ASTNode>> = children.collect();

    let fn_node = FnNode(FnDef {
        name,
        params,
        body: rest,
    });

    Ok(Rc::new(ASTNode::new(fn_node)))
    // aim: return FnDef (name:String, args:Vec<String>, body: Vec<ASTNode>)
}

pub(super) fn parse_if_expression(children: Vec<Rc<ASTNode>>) -> Result<Rc<ASTNode>> {
    if children.len() != 4 {
        let msg = format!(
            "'{}' expected 3 expressions but got {}.",
            IF_NAME,
            children.len()
        );
        return err!(msg);
    }

    let mut children = children.into_iter();
    children.next();

    let cond = children.next().unwrap();
    let e1 = children.next().unwrap();
    let e2 = children.next().unwrap();

    let res = vec![cond, e1, e2];

    let node_val = IfNode(res);
    Ok(Rc::new(ASTNode::new(node_val)))
}

// change to return tuple (ident, expr) since we are checking anyway
pub(super) fn parse_let_expression(
    children: Vec<Rc<ASTNode>>,
    global: bool,
) -> Result<Rc<ASTNode>> {
    // when parsing symbol: do parse atomic, check valid ident
    // else: parse normally
    if children.len() == 1 {
        let msg = format!("'{}' received 0 expressions or symbols", LET_NAME);
        return err!(&msg);
    }

    // remove let
    let mut children = children.into_iter();
    children.next();

    let children = children.collect();

    Ok(Rc::new(ASTNode::new(LetNode(children, global))))
}




#[test]
pub fn parse_let_test() {
    let e1 = "(let x 2)";
    let e2 = "(let x 2 x)";
    let e3 = "(let x 2 y 3 (add x y))";
    let e4 = "(let x 2 (add x y))";
    test_parse(vec![e1, e2, e3, e4]);
}

#[test]
fn parse_if_test() {
    let exprs = vec![
        "(if 1 2 3)",
        "(if (if 0 1 2) (add 5 6) (sub x (mul 4 5)))",
        "(if (3 4 5) (true false) false)",
    ];

    test_parse(exprs);
}

// 0. length should be at least 4 including 'def' (name, args, body)
// 1. first should be a symbol e.g 'fn'
// 2. Next should be an expression (args)
// 3. the expression should only contain symbols
// 4. all the symbols should be valid ident
use lexer::Lexer;
#[test]
fn parse_fn_test_valid() {
    let valid = "(def fn (a) a)";
    let mut l = lex!(valid);

    let valid = "(def fn (a b c) (add a b c) (let x y z) (add 1 2 3))";
    let mut l = lex!(valid);
    assert_eq!(parse(&mut l).unwrap().to_string(), valid.to_string());

    let valid = "def fn (a b c) (add a b c), (let x y z), (add 1 2)";
    let mut l = lex!(valid);
    let p = parse(&mut l).unwrap().to_string();

    let exp = "(def fn (a b c) (add a b c) (let x y z) (add 1 2))";
    assert_eq!(p, exp);

    let valid = "def fn (a,b,c) (add a b c)"; // commas ok
    let mut l = lex!(valid);
    let p = parse(&mut l).unwrap().to_string();
    assert_eq!(p.to_string(), "(def fn (a b c) (add a b c))")
}

#[test]
fn parse_fn_test() {
    let mut l = lex!("def fn (a)");
    assert!(parse(&mut l)
        .err()
        .unwrap()
        .format_error()
        .contains("at least 3"));

    let mut l = lex!("(def fn (a))");
    assert!(parse(&mut l)
        .err()
        .unwrap()
        .format_error()
        .contains("at least 3"));

    let mut l = lex!("(def 2 (a) (a b))");
    assert!(parse(&mut l)
        .err()
        .unwrap()
        .format_error()
        .contains("should be a symbol"));

    let mut l = lex!("(def fn a b (a b))");
    let p = parse(&mut l);
    dbg!(&p);
    // assert!(parse(&mut l).err().unwrap().format_error().contains("in an expression"));

    let mut l = lex!("(def fn (a b 2) (a b))");
    assert!(parse(&mut l)
        .err()
        .unwrap()
        .format_error()
        .contains("only symbols"));

    let mut l = lex!("(def fn (a b let) (add a b let))");
    assert!(parse(&mut l).is_err());
}
