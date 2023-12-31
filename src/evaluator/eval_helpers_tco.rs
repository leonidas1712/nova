use core::num;
use std::rc::Rc;

use std::collections::VecDeque;

use crate::message::*;
use crate::parser::parse_node::*;
use crate::{
    constants::{LET_NAME, STMT_END},
    evaluator::function_tco::UserFunction,
};

use super::evaluator_tco::*;
use super::{context_tco::*, data_tco::*, function_tco::*};

use std::cell::RefCell;
thread_local! {
    pub (crate) static MAX_CALL_LEN: RefCell<usize> = RefCell::new(0);
    pub (crate) static MAX_FN_LEN:RefCell<usize> = RefCell::new(0);
}

// memory metrics
pub fn update_max_len(n: usize) {
    MAX_CALL_LEN.with(|x| {
        let mut rf = x.borrow_mut();
        let val = *rf;
        *rf = n.max(val);
    });
}

pub fn print_max_len() {
    MAX_CALL_LEN.with(|x| {
        let rf = x.borrow();
        let val = *rf;
        println!("Max call stack len:{}", val);
    });
}

pub fn update_max_len_fn(n: usize) {
    MAX_FN_LEN.with(|x| {
        let mut rf = x.borrow_mut();
        let val = *rf;
        *rf = n.max(val);
    });
}

pub fn print_max_len_fn() {
    MAX_FN_LEN.with(|x| {
        let rf = x.borrow();
        let val = *rf;
        println!("Max fn stack len:{}", val);
    });
}

// evaluated args
pub fn get_eval_args_from_nodes(
    iter: impl Iterator<Item = Result<DataValue>> + Clone,
) -> Result<Vec<Arg>> {
    let res_iter: Result<Vec<DataValue>> = iter.clone().into_iter().collect();
    let results = res_iter.map(|v| {
        let args: Vec<Arg> = v.into_iter().map(|x| Arg::Evaluated(x)).collect();
        return args;
    })?;
    Ok(results)
}

// get args from results queue for func
fn get_args(func: &FunctionCall, results: &mut VecDeque<ExpressionResult>) -> Vec<Arg> {
    let mut args: VecDeque<Arg> = VecDeque::new();

    // take from back of results queue until we encounter res with diff parent
    for res in results.iter().rev() {
        if !can_resolve(func, &res.parent) {
            break;
        }

        let data = res.data.clone();
        let arg = Arg::Evaluated(data);
        args.push_front(arg);
    }

    // println!("before:{}", results.len());
    // pop after pushing: can't modify during iter
    for _i in 0..args.len() {
        results.pop_back();
    }
    // println!("after:{}", results.len());

    args.into_iter().collect()
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
        let msg = format!("Invalid identifier -  '{}'", s);
        return err!(&msg);
    } else if RESERVED_SET.contains(&s) {
        let msg = format!("Invalid identifier - '{}' is a reserved keyword.", s);
        return err!(&msg);
    }

    Ok(s)
}

// checks if func call ast and parent are the same
pub fn can_resolve(fn_call: &FunctionCall, expr_parent: &Option<Rc<ASTNode>>) -> bool {
    let fn_ast = &fn_call.ast;

    match expr_parent {
        Some(parent) => {
            // check nodes directly
            parent.as_ref().eq(fn_ast.as_ref())
        }
        None => false,
    }
}

// for (def fn ...)
pub fn resolve_fn_node(ctx: &EvalContext, fn_def: &FnDef, outer_call: bool) -> Result<DataValue> {
    let func = UserFunction::new(ctx, &fn_def);
    let rc: Rc<UserFunction> = Rc::new(func);

    if !outer_call {
        return Ok(FunctionVariable(rc));
    }

    // to return out a function to set in global variable
    Ok(SetFn(rc))
}

pub fn resolve_let(
    ctx: &EvalContext,
    expressions: &Vec<Rc<ASTNode>>,
    global: bool,
) -> Result<DataValue> {
    let mut new_ctx = ctx.copy(); // copy, not clone
    let n = expressions.len();

    let mut var: Option<&str> = None;
    let mut outer_res: Option<DataValue> = None;

    for (idx, nxt_node) in expressions.into_iter().enumerate() {
        if idx == n - 1 {
            let res = evaluate_outer(new_ctx.clone(), Rc::clone(nxt_node), false)?;
            if let Some(var_name) = var {
                new_ctx.write().add_variable(var_name, res.clone());
            }

            outer_res.replace(res);
            continue;
        }

        // assign result to var name
        if var.is_some() {
            let res = evaluate_outer(new_ctx.clone(), Rc::clone(nxt_node), false)?;
            outer_res.replace(res.clone());
            new_ctx.write().add_variable(var.unwrap(), res);
            var.take();
            continue;
        }

        match &nxt_node.value {
            Symbol(string) => {
                if string.as_str().eq(STMT_END) {
                    let msg = format!("'{}' can't be used here.", STMT_END);
                    return err!(msg);
                }
                let _check = is_valid_identifier(string.as_str())?;
                var.replace(string.as_str());
            }
            _ => {
                let msg = format!(
                    "'{}' expected a symbol but got '{}'",
                    LET_NAME,
                    nxt_node.to_string()
                );
                return err!(&msg);
            }
        }
    }

    if outer_res.is_none() {
        let msg = format!("'{}' received nothing to evaluate.", LET_NAME);
        return err!(&msg);
    }

    let res = outer_res.unwrap();

    if !global {
        Ok(res)
    } else {
        let data = LetReturn::new(new_ctx, res);
        Ok(SetVar(data))
    }
}

pub struct ResolveExprArgs<'a> {
    pub ast: &'a Rc<ASTNode>,           // ast for the function call
    pub children: &'a Vec<Rc<ASTNode>>, // all the children of the expression
    pub ctx: &'a EvalContext,
    pub parent: &'a Option<Rc<ASTNode>>, // parent of the expression
}

// unroll expression onto call stack and resolve first member to a function then push to fn_stack
pub fn resolve_expression(
    call_stack: &mut VecDeque<StackExpression>,
    fn_stack: &mut VecDeque<FunctionCall>,
    results: &mut VecDeque<ExpressionResult>,
    args: ResolveExprArgs,
) -> Result<()> {
    let children = args.children;
    let ctx = args.ctx;
    let parent = args.parent;
    let ast = args.ast;

    if children.is_empty() {
        return err!("Received empty expression.");
    }

    let first_child = children.first().unwrap();
    // recursively evaluate function call
    let eval_first = evaluate_outer(ctx.clone(), Rc::clone(first_child), false)?;

    // we expect first part of expression to resolve to a fn call
    // let and if handled separately already
    let func = eval_first.expect_function()?;
    let func_call = FunctionCall {
        func: func.clone(),
        ast: Rc::clone(ast),
        parent: parent.clone(),
        context: ctx.clone(),
    };

    // push rest of child expressions onto call_st
    let mut rest_children = children.into_iter();
    rest_children.next(); // go past first

    // uneval: dont use stack, pass args directly to function
    if func.get_arg_type().eq(&ArgType::Unevaluated) {
        let args: Vec<Arg> = rest_children.map(|x| Unevaluated(Rc::clone(x))).collect();
        return evaluate_fn(args, &func_call, call_stack, results, fn_stack);
    }

    fn_stack.push_back(func_call);

    // update_max_len_fn(fn_stack.len());

    // push in reverse
    for child in rest_children.rev() {
        if child.is_unit() {
            continue;
        }

        let deferred = DeferredExpression {
            ctx: ctx.clone(),
            body: Rc::clone(child),
        };
        let stack_expr = StackExpression {
            expr: deferred,
            parent: Some(Rc::clone(ast)),
        };

        // assigned parent to one level above supposed to be

        call_stack.push_back(stack_expr);
        // update_max_len(call_stack.len());
    }

    Ok(())
}

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
            body: Rc::new(e1.as_ref().clone()),
        })
    } else {
        Ok(DeferredExpression {
            ctx: ctx.clone(),
            body: Rc::new(e2.as_ref().clone()),
        })
    }
}
// change: func.execute should always return a curried function
// func.resolve forces the function to return the actual value
// for finite: return the function as is if args < needed, else return the eval value
// inf: return func if args < min, else return eval value
pub fn evaluate_fn(
    args: Vec<Arg>,
    func_call: &FunctionCall,
    call_stack: &mut VecDeque<StackExpression>,
    results: &mut VecDeque<ExpressionResult>,
    fn_stack: &mut VecDeque<FunctionCall>,
) -> Result<()> {
    // if args.len() == 0 {
    //     let msg = format!("'{}' received 0 arguments.", func_call.func.to_string());
    //     return err!(msg);
    // }

    // 0. call func.apply
    // 1. if func.ast is_func (result expected to be function)
    // put on res q
    // 2. if func.ast NOT is_func (result expected to be final result) -> call func.resolve
    // put on call_stack or result as normal

    let func = func_call.func.apply(&args);
    if func_call.ast.is_func {
        let expr_res = ExpressionResult {
            data: FunctionVariable(func),
            parent: func_call.parent.clone(), // cloning the OPTION - should be same id
        };

        results.push_back(expr_res);
        return Ok(());
    }

    let execute_result = func.resolve(&func_call.context)?;

    // println!("Result (not is_func):{}", execute_result.to_string());

    // println!("AST during eval:{}", func_call.ast.to_string_with_parent());
    match execute_result {
        // put on call stack
        DeferredExpr(def) => {
            let stack_expr = StackExpression {
                expr: def,
                parent: func_call.parent.clone(), // cloning the OPTION
            };
            call_stack.push_back(stack_expr);
        }

        // put on resq
        EvaluatedExpr(ev) => {
            let expr_res = ExpressionResult {
                data: ev,
                parent: func_call.parent.clone(), // cloning the OPTION - should be same id
            };
            results.push_back(expr_res);
        }
    }

    Ok(())
}

// call function that takes evaluated arguments (args are on the res_q)
pub fn call_fn_evaluated(
    fn_stack: &mut VecDeque<FunctionCall>,
    call_stack: &mut VecDeque<StackExpression>,
    results: &mut VecDeque<ExpressionResult>,
) -> Result<()> {
    let func = &fn_stack.pop_back().unwrap();
    let args = get_args(func, results);

    evaluate_fn(args, func, call_stack, results, fn_stack)
}
