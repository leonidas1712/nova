use std::rc::Rc;

use std::collections::VecDeque;

use crate::parser::parse_node::*;
use crate::{message::*};


use super::{context_tco::*, data_tco::*, eval_helpers_tco::*, function_tco::*};

// FnDef: returns ExpressionResult
// if: returns DeferredExpr
// let: returns ExpressionResult (or Deferred)
// evaluate before returning
// alt: take the last expr + ctx out and put on stack as deferred

// represents Expression: either deferred or evaluated
// need to separate because builtin functions have no ASTNode to return
#[derive(Display, Clone)]
pub enum Expression {
    DeferredExpr(DeferredExpression),
    EvaluatedExpr(DataValue),
}

pub use Expression::*;

// function call on the call stack
#[derive(Clone)]
pub struct FunctionCall {
    pub func: Rc<dyn Function>,
    pub ast: Rc<ASTNode>,
    pub parent: Option<Rc<ASTNode>>,
    pub context:EvalContext
}

// an expression on the call stack
// separate struct because parent pointer is always set after returning from another function
// so inside a sub-function we can just return part of it and centralise setting of the parent ptr
// ensuring that a deferredexpr without a parent is invalid and wont go on the stack
pub struct StackExpression {
    pub expr: DeferredExpression,
    pub parent: Option<Rc<ASTNode>>,
}

// body: used for eval, parent: used for checking
#[derive(Clone)]
pub struct DeferredExpression {
    pub ctx: EvalContext,
    pub body: Rc<ASTNode>,
}

// result on the result queue
// ensuring evaluated expr without parent is invalid, doesnt go on resq
#[derive(Clone)]
pub struct ExpressionResult {
    pub data: DataValue,
    pub parent: Option<Rc<ASTNode>>,
}

// (def recr (n) (if (eq n 0) 0 (add n (recr (pred n)))))
// (def recr (n) (if (eq n 0) 0 (recr (pred n))))
use std::cell::RefCell;
thread_local! {
    pub (crate) static DEPTH_TCO: RefCell<u64> = RefCell::new(0);
    pub (crate) static MAX_DEPTH_TCO:RefCell<u64>=RefCell::new(0);
}

// depth tracking 
pub (crate) fn update_depth_tco() {
    DEPTH_TCO.with(|x| {
        let mut rf = x.borrow_mut();
        let val = *rf;
        *rf = val + 1;

        MAX_DEPTH_TCO.with(|r| {
            let mut max_depth = r.borrow_mut();
            let max_value = *max_depth;
            *max_depth = max_value.max(val + 1);

            // println!("Max depth: {}", *max_depth);
        });
    });
}

pub (crate) fn subtract_depth_tco() {
    DEPTH_TCO.with(|x| {
        let mut rf = x.borrow_mut();
        let val = *rf;
        *rf = val - 1;
        // println!("Subtracted depth:{}", *rf);
    });
}

pub (crate) fn print_max_depth_tco() {
    MAX_DEPTH_TCO.with(|x|{
        let rf=x.borrow();
        println!("Max depth:{}", *rf);
    });
}

// main evaluate
pub(crate) fn evaluate_outer(ctx: EvalContext,node: Rc<ASTNode>, outer_call: bool,) -> Result<DataValue> {
    // try to match terminals

    // update_depth_tco();
    let deferred = DeferredExpression {
        ctx: ctx.clone(),
        body: Rc::clone(&node),
    };

    let stack_expr = StackExpression {
        expr: deferred,
        parent: node.parent.clone(),
    };

    let res=evaluate_tco(stack_expr, outer_call);
    // subtract_depth_tco();
    res
}

fn resolve(call_stack: &mut VecDeque<StackExpression>, fn_stack: &mut VecDeque<FunctionCall>,
    results: &mut VecDeque<ExpressionResult>,outer_call: bool)
 -> Result<()> {
    // pop from stack
    let expression = call_stack.pop_back().unwrap();
    let expr = &expression.expr;

    let body = &expr.body;
    let ctx = &expr.ctx;
    let parent=&expression.parent; // dont use body.parent

    let mut result = ExpressionResult {
        data: Num(-1),
        parent: parent.clone(), // so I don't have to set parent repeatedly
    };

    match &body.value {
        Number(n) => {
            result.data = Num(*n);
            results.push_back(result);
        }
        Boolean(b) => {
            result.data = Bool(*b);
            results.push_back(result);
        }
        Symbol(sym) => {
            let read = ctx.read();
            let value = read.get_data_value(&sym);

            match value {
                Some(val) => {
                    result.data = val.clone();
                    results.push_back(result);
                }
                None => {
                    let err_string = format!("Unrecognised symbol: '{}'", sym);
                    return err!(err_string.as_str());
                }
            }
        }
        IfNode(children) => {
            let res = evaluate_if(ctx, children)?;
            let stack_expr = StackExpression {
                expr: res,
                parent: parent.clone(),
            };
            call_stack.push_back(stack_expr);
        },
        // only a side effect, no return (besides err)
        ParseExpression(children) => {
            let args=ResolveExprArgs {
                children,
                ctx,
                parent,
                ast:body
            };
            resolve_expression(call_stack, fn_stack, results, args)?;
        },
        LetNode(children, global) => {
            let returned_result=resolve_let(&ctx,children,*global)?;
            result.data=returned_result;
            results.push_back(result);
        },
        FnNode(fn_def) => {
            let fn_resolve=resolve_fn_node(&ctx, &fn_def, outer_call)?;
            result.data=fn_resolve;
            results.push_back(result);
        }
        // List
        _ => {
            todo!()
        }
    }

    Ok(())
}

fn evaluate_tco(expression: StackExpression, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    let mut call_stack: VecDeque<StackExpression> = VecDeque::new();
    let mut fn_stack: VecDeque<FunctionCall> = VecDeque::new();
    let mut results_queue: VecDeque<ExpressionResult> = VecDeque::new();

    let _max_len=0;

    let expr_string = &expression.expr.body.to_string();
    call_stack.push_back(expression);

    // what to do with expression on call_st when valid
    // valid: fn_st empty and call_st not empty OR fn_st[-1].ast==call_st[-1].parent

    // call_st only: unroll the expr
    // fn_stack only: check res_q
    // both: check fn_st[-1].ast vs call_st[-1].parent
    while !call_stack.is_empty() || !fn_stack.is_empty() {
        // update_max_len(call_stack.len());

        let call_has = !call_stack.is_empty();
        let fn_has = !fn_stack.is_empty();

        // both - check ast vs parent
        if call_has && fn_has {
            let call_st_last=call_stack.back().unwrap();
            let fn_st_last=fn_stack.back().unwrap();
            
            if can_resolve(fn_st_last, &call_st_last.parent) {
                resolve(&mut call_stack, &mut fn_stack, &mut results_queue,outer_call)?;
                // update_max_len(call_stack.len());
                
            // when call_stack[-1] doesnt match fn_st[-1]: evaluate
            } else {
                evaluate_fn(&mut fn_stack, &mut call_stack, &mut results_queue)?;
            }
        }

        // call only: resolve whats on it
        else if call_has && !fn_has {
            resolve(&mut call_stack, &mut fn_stack, &mut results_queue,outer_call)?;
            // update_max_len(call_stack.len());

        }

        // fn only - fn.execute
            // 1. get correct args from result queue
            // 2. pass to fn execute, get Expression
            // 3. push onto res_q with correct parent=fn_ast.parent
        else {
            evaluate_fn(&mut fn_stack, &mut call_stack, &mut results_queue)?;
        }

        // update_max_len(call_stack.len());

    }

    // (def recr (n) (if (eq n 0) 0 (recr (pred n))))
    // (def recr (n) (if (eq n 0) 0 (add n (recr (pred n)))))
    // tail: (def recr_t (n acc) (if (eq n 0) acc (recr_t (pred n) (add acc n))))
        // works: fn stack len doesnt go past 2


    match results_queue.into_iter().last() {
        Some(res) =>{ 
            Ok(res.data) 
        },
        None => {
            let msg = format!("Could not evaluate expression: {}", expr_string);
            return err!(msg);
        }
    }
}
