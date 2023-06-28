use crate::constants::*;
use crate::lexer;
use crate::message::*;
use crate::parser::node::*;
use super::parser::*;

pub (super) fn parse_fn_def(lex: &mut lexer::Lexer)->Result<ASTNode> {
    lex.next();
    Ok(ASTNode::new(Symbol("FnDef".to_string())))
}

pub (super) fn parse_if_expression(lex: &mut lexer::Lexer)->Result<ASTNode> {
    lex.next();
    let cond=parse_expression(lex);
    let e1=parse_expression(lex);
    let e2=parse_expression(lex);

    let all:Vec<Result<ASTNode>> = vec![cond,e1,e2];
    let checked:Result<Vec<ASTNode>>=all.into_iter().collect();

    let res=checked?;

    let last=lex.peek();

    match last {
        Some(token) if !token.eq(CLOSE_EXPR)=> {
            return Err(Ex::new("'if' received too many expressions."))
        },
        _ => ()
    }

    let node_val=IfNode(res);
    Ok(ASTNode::new(node_val))
}


pub (super) fn parse_let_expression(lex: &mut lexer::Lexer)->Result<ASTNode> {
    lex.next();
    Ok(ASTNode::new(Symbol("LetStmt".to_string())))
}