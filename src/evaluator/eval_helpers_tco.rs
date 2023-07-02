use std::rc::Rc;

use super::evaluator_tco::*;
use super::{context_tco::*, data_tco::*};
use crate::lex;
use crate::message::*;
use crate::parser::parse_node::*;
use crate::{
    constants::{DONT_ADD, LET_NAME, RESERVED_KEYWORDS, SPLIT_TOKENS},
    evaluator::function_tco::UserFunction,
    parser::parser::tests::test_parse,
};

// evaluated args
pub fn get_eval_args_from_nodes<'a>(
    iter: impl Iterator<Item = Result<DataValue>> + Clone,
) -> Result<Vec<Arg<'a>>> {
    let res_iter: Result<Vec<DataValue>> = iter.clone().into_iter().collect();
    let results = res_iter.map(|v| {
        let args: Vec<Arg> = v.into_iter().map(|x| Arg::Evaluated(x)).collect();
        return args;
    })?;
    Ok(results)
}

use crate::constants::{INVALID_SET, RESERVED_SET};

// returns Result so I can unwrap using ?
pub fn is_valid_identifier(s: &str) -> Result<String> {
    let s = &s;
    let try_num: std::result::Result<i64, _> = s.parse();

    if try_num.is_ok() {
        let msg = format!("Invalid identifier - '{}' is a number. ", s);
        return err!(&msg);
    }

    let s = s.to_string();

    if INVALID_SET.contains(&s) {
        let msg = format!("Invalid identifier -  {}", s);
        return err!(&msg);
    } else if RESERVED_SET.contains(&s) {
        let msg = format!("Invalid identifier - {}' is a reserved keyword.", s);
        return err!(&msg);
    }

    Ok(s)
}

// evaluate first child. if len==1, return
// elif first child is FnVar | FnDef => apply to arguments
// else: evaluate nodes in order, return result from last
// pub fn evaluate_expression(ctx: &EvalContext, children: &Vec<Rc<ASTNode>>) -> Result<DataValue> {
//     if children.is_empty() {
//         return err!("Received empty expression.");
//     }

//     let first_child = children.first().unwrap();
//     let res = eval!(ctx.clone(), Rc::clone(first_child))?;

//     if children.len() == 1 {
//         return Ok(res);
//     }
//     // cant use same ref from UF exec because we use body to unroll: need to clone and change ref
//     let mut rest = children.iter();
//     rest.next();

//     let eval_rest = rest.clone().map(|node| eval!(ctx.clone(), Rc::clone(node)));

//     // is function: check ArgType, gets arg, eval.
//         // if err: insert eval_rest.clone() again
//     match res.expect_function().ok() {
//         Some(func) => {
//             if func.get_arg_type() == ArgType::Evaluated {
//                 let results = get_eval_args_from_nodes(eval_rest)?;
//                 // has to return out the merged_ctx in DeferredExpr
//                     // but merged_ctx is local
//                 func.execute(results, &ctx)
//             } else {
//                 // just ast nodes
//                 let args: Vec<Arg> = children.into_iter().map(|x| Unevaluated(x)).collect();
//                 func.execute(args, &ctx)
//             }
//         }
//         // not a function: evaluate in order and return last
//         None => {
//             let res_iter: Result<Vec<DataValue>> = eval_rest.into_iter().collect();
//             res_iter?
//                 .into_iter()
//                 .last()
//                 .ok_or(Ex::new("Couldn't evaluate expression."))
//         }
//     }
// }

// pub fn evaluate_list(_ctx: &EvalContext, children: &Vec<Rc<ASTNode>>) -> Result<DataValue> {
//     dbg!(children);
//     Ok(Default)
// }
// DeferredExpr: body=returned condition,
//
//
pub fn evaluate_if(ctx: &EvalContext, children: &Vec<Rc<ASTNode>>) -> Result<DeferredExpression> {
    // recursive eval: real recursion
    let cond = children.get(0).unwrap();
    let e1 = children.get(1).unwrap();
    let e2 = children.get(2).unwrap();

    let cond_result = evaluate_outer(ctx.clone(), Rc::clone(cond), false)?;

    // add empty list as false later
    let condition = match cond_result {
        Num(num) => num != 0,
        Bool(b) => b,
        _ => true,
    };

    if condition {
        Ok(DeferredExpression {
            ctx: ctx.clone(),
            body: e1.clone(),
        })
    } else {
        Ok(DeferredExpression {
            ctx: ctx.clone(),
            body: e2.clone(),
        })
    }
}

// pub fn evaluate_let(
//     ctx: &EvalContext,
//     expressions: &Vec<Rc<ASTNode>>,
//     outer_call: bool,
// ) -> Result<DataValue> {
//     let mut new_ctx = ctx.copy(); // copy: new shouldn't affect old
//     let n = expressions.len();

//     let mut var: Option<&str> = None; // name of var to set in map

//     // if var is None: expect symbol to assign
//     // if var is Some: evaluate
//     let mut outer_res: Option<DataValue> = None;

//     for (idx, nxt_node) in expressions.into_iter().enumerate() {
//         if idx == n - 1 {
//             let res = eval!(new_ctx.clone(), Rc::clone(nxt_node))?;

//             if let Some(var_name) = var {
//                 new_ctx.write().add_variable(var_name, res.clone());
//             }

//             outer_res.replace(res);
//             continue;
//         }

//         if var.is_some() {
//             let res = eval!(new_ctx.clone(), Rc::clone(nxt_node))?;
//             outer_res.replace(res.clone());

//             new_ctx.write().add_variable(var.unwrap(), res);
//             var.take();
//             continue;
//         }

//         // dont check last expression as var
//         if idx == n - 1 {
//             continue;
//         }

//         // None: expect symbol
//         match &nxt_node.value {
//             Symbol(string) => {
//                 let check = is_valid_identifier(string.as_str())?;
//                 var.replace(string.as_str());
//             }
//             _ => {
//                 let msg = format!(
//                     "'{}' expected a symbol but got '{}'",
//                     LET_NAME,
//                     nxt_node.to_string()
//                 );
//                 return err!(&msg);
//             }
//         }
//     }

//     // (let ) -> unrecognised

//     if outer_res.is_none() {
//         let msg = format!("'{}' received nothing to evaluate.", LET_NAME);
//         return err!(&msg);
//     }

//     let res = outer_res.unwrap();

//     // returned here
//     if !outer_call {
//         Ok(res)
//     } else {
//         let data = LetReturn::new(new_ctx, res);
//         Ok(SetVar(data))
//     }
// }

use super::function::*;
use crate::parser::parse_node::FnDef;
pub fn evaluate_fn_node(ctx: &EvalContext, fn_def: &FnDef, outer_call: bool) -> Result<DataValue> {
    let func = UserFunction::new(ctx, &fn_def);
    let rc: Rc<UserFunction> = Rc::new(func);

    if !outer_call {
        return Ok(FunctionVariable(rc));
    }

    // to return out a function to set in global variable
    Ok(SetFn(rc))
}
