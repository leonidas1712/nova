use crate::constants::*;

use crate::evaluator::eval_helpers::is_valid_identifier;
use crate::lexer;
use crate::message::*;
use crate::parser::parse_node::*;
use super::parser::*;

pub (super) fn parse_special(spec_type: Special, children: Vec<ASTNode>, global:bool)->Result<ASTNode> {
    match spec_type {
        Special::If => return parse_if_expression(children),
        Special::Let => return parse_let_expression(children, global),
        Special::Fn => return parse_fn_def(children, global)
    }
}

// create FnNode
    // def, name, args, body
pub (super) fn parse_fn_def(children: Vec<ASTNode>, global:bool)->Result<ASTNode> {
    // children.iter().for_each(|n| println!("{}", n.to_string()));
    if children.len() < 4 {
        return err!("Function definitions should have at least 3 parts: a name, parameters and a body.");
    }

    let mut children=children.into_iter();
    children.next(); // move past def

    let name=children.next().unwrap().get_symbol();
    
    if name.is_none() {
        return err!("Function name should be a symbol.");
    }

    let name=name.unwrap();

    // should be inside expression
    let params=children.next().unwrap().get_expression();
    if params.is_none() {
        let msg=format!("Parameters for '{}' should be in an expression.",name);
        return err!("Function parameters should be in an expression.");
    }

    let params=params.unwrap();

    let all_symbols:Option<Vec<String>>=params.iter().map(|p| p.get_symbol()).collect();
    if all_symbols.is_none() {
        return err!("Function parameters should contain only symbols");
    }

    let all_symbols=all_symbols.unwrap();
    let all_ok:Result<Vec<String>>=all_symbols.iter().map(|s| is_valid_identifier(s)).collect();

    let params=all_ok?;

    // end of err handling
    let rest:Vec<ASTNode>=children.collect();

    let fn_node=FnNode(FnDef {
        name,
        params,
        body:rest
    });

    Ok(ASTNode::new(fn_node))
    // aim: return FnDef (name:String, args:Vec<String>, body: Vec<ASTNode>)
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
pub (super) fn parse_let_expression(children: Vec<ASTNode>, global:bool)->Result<ASTNode> {
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

    Ok(ASTNode::new(LetNode(children, global)))
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
fn parse_if_test() {
    let exprs=vec![
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

#[test]
fn parse_fn_test() {
    let l=lex!("def fn (a)");
    assert!(parse(l).err().unwrap().format_error().contains("at least 3"));

    let l=lex!("(def fn (a))");
    assert!(parse(l).err().unwrap().format_error().contains("at least 3"));

    let l=lex!("(def 2 (a) (a b))");
    assert!(parse(l).err().unwrap().format_error().contains("should be a symbol"));

    let l=lex!("(def fn a b (a b))");
    assert!(parse(l).err().unwrap().format_error().contains("in an expression"));

    let l=lex!("(def fn (a b 2) (a b))");
    assert!(parse(l).err().unwrap().format_error().contains("only symbols"));

    let l=lex!("(def fn (a b let) (add a b let))");
    assert!(parse(l).is_err());

    let valid="(def fn (a b c) (add a b c) (let x y z) (add 1 2 3))";
    let l=lex!(valid);
    assert_eq!(parse(l).unwrap().to_string(),valid.to_string());


    let valid="def fn (a b c) (add a b c) (let x y z) (add 1 2)";
    let l=lex!(valid);
    let p=parse(l).unwrap().to_string();
    let exp="(def fn (a b c) (add a b c) (let x y z) (add 1 2))";
    assert_eq!(p, exp);

}