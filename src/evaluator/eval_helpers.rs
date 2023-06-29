use super::{context::*, data::*, evaluator::evaluate};
use crate::constants::{DONT_ADD, RESERVED_KEYWORDS, SPLIT_TOKENS, LET_NAME};
use crate::{message::*};
use crate::parser::parse_node::*;

// evaluated args
pub fn get_eval_args_from_nodes<'a>(
    iter: impl Iterator<Item = Result<DataValue>> + Clone,
) -> Result<Vec<Arg<'a>>> {
    // let eval_rest=nodes.iter().map(|node| evaluate(ctx, node));
    let res_iter: Result<Vec<DataValue>> = iter.clone().into_iter().collect();
    let results = res_iter.map(|v| {
        let args: Vec<Arg> = v.into_iter().map(|x| Arg::Evaluated(x)).collect();
        return args;
    })?;
    Ok(results)
}

pub fn is_valid_identifier(s:&str)->Result<bool> {
    let s=&s;
    if DONT_ADD.contains(s) || SPLIT_TOKENS.contains(s) {
        let msg=format!("Invalid identifier: {}", s);
        return err!(&msg);
    } else if RESERVED_KEYWORDS.contains(s) {
        let msg=format!("Invalid identifier: '{}' is a reserved keyword.", s);
        return err!(&msg);
    }

    Ok(true)
}

// evaluate first child. if len==1, return
// elif first child is FnVar | FnDef => apply to arguments
// else: evaluate nodes in order, return result from last
pub fn evaluate_expression(ctx: &Context, children: &Vec<ASTNode>) -> Result<DataValue> {
    if children.is_empty() {
        return err!("Received empty expression.");
    }

    let first_child = children.first().unwrap();
    let res = evaluate(ctx, first_child)?;

    if children.len() == 1 {
        return Ok(res);
    }

    let mut rest = children.iter();
    rest.next();

    let eval_rest = rest.clone().map(|node| evaluate(ctx, node));

    // is function: check ArgType, gets arg, eval.
    match res.expect_function().ok() {
        Some(func) => {
            if func.get_arg_type() == ArgType::Evaluated {
                let results = get_eval_args_from_nodes(eval_rest.clone())?;
                func.execute(results, ctx)

                // let strings:Vec<String>=results.into_iter().map(|x| x.to_string()).collect();
                // dbg!(strings);
            } else {
                // just ast nodes
                let args: Vec<Arg> = children.into_iter().map(|x| Unevaluated(x)).collect();
                func.execute(args, ctx)
            }
        }
        // not a function: evaluate in order and return last
        None => {
            let res_iter: Result<Vec<DataValue>> = eval_rest.clone().into_iter().collect();
            res_iter?
                .into_iter()
                .last()
                .ok_or(Ex::new("Couldn't evaluate expression."))
        }
    }
}

pub fn evaluate_list(_ctx: &Context, children: &Vec<ASTNode>) -> Result<DataValue> {
    println!("list eval");
    dbg!(children);
    Ok(Default)
}

pub fn evaluate_if(ctx: &Context, cond: &ASTNode, e1: &ASTNode, e2: &ASTNode) -> Result<DataValue> {
    // println!("Received if eval: Cond: {} e1: {} e2: {}", cond.to_string(), e1.to_string(), e2.to_string());
    let cond_result = evaluate(ctx, cond)?;

    // add empty list as false later
    let condition = match cond_result {
        Num(num) => num != 0,
        Bool(b) => b,
        _ => true,
    };

    if condition {
        evaluate(ctx, e1)
    } else {
        evaluate(ctx, e2)
    }
}

pub fn evaluate_let(ctx: &Context, expressions: &Vec<ASTNode>) -> Result<DataValue> {
    // println!("Let received eval:", expressions.);
    // expressions.iter().for_each(|n| println!("{}", n.to_string()));
    let mut new_ctx=ctx.clone();
    let n=expressions.len();

    let mut var:Option<&str>=None; // name of var to set in map

    // if var is None: expect symbol to assign
    // if var is Some: evaluate

    for (idx,nxt_node) in expressions.into_iter().enumerate() {
        if idx == n-1 {
            return evaluate(&new_ctx, nxt_node);
        }

        if var.is_some() {
            let res=evaluate(&new_ctx, &nxt_node)?;
            new_ctx.add_variable(var.unwrap(), res);
            var.take();
            continue;
        }

        // None: expect symbol
        match &nxt_node.value {
            Symbol(string) => {
                let check=is_valid_identifier(string.as_str())?;
                var.replace(string.as_str());
            }
            _ => {
                let msg=format!("'{}' expected a symbol but got '{}'", LET_NAME, nxt_node.to_string());
                return err!(&msg);
            }
        }
    }

    Ok(Default)
}

use crate::lexer::Lexer;
#[test]
fn let_test() {
    let e="(let if 2)";
  
}


