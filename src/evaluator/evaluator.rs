use super::{context::*, data::*};
use crate::parser::*;
use crate::parser::node::*;
use crate::message::*;


pub fn evaluate(ctx:&Context, node:&ASTNode, user_fn:bool)->Result<DataValue> {
    let mut nova_result=NovaResult::new(DataValue::Default);

    if user_fn {
        println!("Called by user function:{}", node.value.to_string());
        return Ok(nova_result)
    }
    // placeholder
    

    println!("eval");
    context();

    Ok(nova_result)
}