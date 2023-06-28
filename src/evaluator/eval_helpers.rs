use super::{context::*, data::*, evaluator::evaluate};
use crate::message::*;
use crate::parser::node::*;

pub fn get_args_from_nodes(iter:impl Iterator<Item=Result<DataValue>> + Clone)->Result<Vec<Arg>> {
    // let eval_rest=nodes.iter().map(|node| evaluate(ctx, node));
    let res_iter:Result<Vec<DataValue>>=iter.clone().into_iter().collect();
    let results=res_iter.map(|v| {
        let args:Vec<Arg>=v.into_iter().map(|x| Arg::Evaluated(x)).collect();
        return args;
    })?;
    Ok(results)
}
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
    let mut rest=children.iter();
    rest.next();

    let eval_rest=rest.clone().map(|node| evaluate(ctx, node));
 
    // is function: check ArgType, gets arg, eval.
    match res.get_function() {
        Some(func) => {
            if func.get_arg_type()==ArgType::Evaluated {
                let results= get_args_from_nodes(eval_rest.clone())?;
                
                // let strings:Vec<String>=results.into_iter().map(|x| x.to_string()).collect();
                // dbg!(strings);
            }
            
        },
        None => {
            println!("got to none");
        }
    }

    // iterate over rest, convert to data value, then arg,  propagate errors
        // -> either get iter of Args or an err

    println!("expr");
    dbg!(children);
    Ok(Num(900))
}

pub fn evaluate_list(ctx:&Context, children:&Vec<ASTNode>)->Result<DataValue> {
    println!("list eval");
    dbg!(children);
    Ok(Default)
}