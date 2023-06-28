use crate::constants::*;
use crate::evaluator::eval_helpers::is_valid_identifier;
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


// change to return tuple (ident, expr) since we are checking anyway
pub (super) fn parse_let_expression(lex: &mut lexer::Lexer)->Result<ASTNode> {
    lex.next();

    let mut ident:Option<String>=None;

    // when parsing symbol: do parse atomic, check valid ident
    // else: parse normally
    let mut children:Vec<(String,ASTNode)>=vec![];

    while let Some(token)=lex.peek() {
        if token==CLOSE_EXPR {
            break;
        }

        if ident.is_some() {
            let res=parse_expression(lex)?;
            children.push((ident.unwrap(), res));

            continue;
        }

        // None: use parse atomic
        let res=parse_atomic_expression(lex)?;

        match &res.value {
            Symbol(string) => {
                is_valid_identifier(string)?;
                ident.replace(string.to_string());

            },
            _ => {
                let msg=format!("'let' expected identfier but got: {}", res.to_string());
                return Err(Ex::new(&msg));
            }
        }
        
    }

    if children.len()==0 {
        let msg=format!("'{}' received 0 expressions or symbols", LET_NAME);
        return Err(Ex::new(&msg));
    }

    Ok(ASTNode::new(LetNode(children)))
}

#[test]
fn let_test() {
    let lex=lexer::Lexer::new("(let x 2)".to_string()).unwrap();
    let p=parse(lex);
    dbg!(p);
}