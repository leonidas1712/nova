use super::{context::*, data::*};
use crate::message::*;
use crate::parser::node::*;

pub fn evaluate_expression(ctx:&Context, children:&Vec<ASTNode>)->Result<DataValue> {
    println!("expr");
    dbg!(children);
    Ok(Default)
}

pub fn evaluate_list(ctx:&Context, children:&Vec<ASTNode>)->Result<DataValue> {
    println!("list eval");
    dbg!(children);
    Ok(Default)
}