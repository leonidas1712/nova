use super::{context::*, data::*, evaluator::evaluate};
use crate::message::*;
use crate::parser::node::*;

// evaluate first child. if len==1, return
    // elif first child is FnVar | FnDef => apply to arguments
    // else: evaluate nodes in order, return result from last
pub fn evaluate_expression(ctx:&Context, children:&Vec<ASTNode>)->Result<DataValue> {
    if children.is_empty() {
        return Err(Ex::new("Received empty expression."));
    }

    let first_child=children.first().unwrap();
    let res=evaluate(ctx,first_child)?;

    if children.len()==1 {
        return Ok(res)
    }

    // let v:Vec<Result<u32>>=vec![Ok(1),Ok(2), Ok(1), Err(Ex::new("oops"))];
    // let k:Result<Vec<u32>>=v.into_iter().collect();
 
    // is function: check ArgType, gets arg, eval.
    match res.get_function() {
        Some(func) => {

        },
        None => {

        }
    }

    // iterate over rest, convert to data value, then arg,  propagate errors
        // -> either get iter of Args or an err

    println!("expr");
    dbg!(children);
    Ok(Default)
}

pub fn evaluate_list(ctx:&Context, children:&Vec<ASTNode>)->Result<DataValue> {
    println!("list eval");
    dbg!(children);
    Ok(Default)
}