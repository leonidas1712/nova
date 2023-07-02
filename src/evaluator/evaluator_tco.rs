use std::rc::Rc;

use crate::parser::parse_node::*;
use crate::{evaluate_input, lex, message::*, setup_context};

use super::{context::*, data::*, function::*, eval_helpers_tco::*};


// push results straight to res_q
    // because function call is resolved recursively
    // later: this can change

// FnDef: returns ExpressionResult
// if: returns DeferredExpr
// let: returns ExpressionResult (or Deferred)
    // evaluate before returning
    // alt: take the last expr + ctx out and put on stack as deferred

// represents Expression: either deferred or evaluated
    // need to separate because builtin functions have no ASTNode to return
#[derive(Display,Clone)]
pub enum Expression {
    DeferredExpr(DeferredExpression),
    EvaluatedExpr(DataValue)
}

pub use Expression::*;

// function call on the call stack
#[derive(Clone)]
pub struct FunctionCall {
    pub func:Rc<dyn Function>,
    pub ast:Rc<ASTNode>,
    pub parent:Option<Rc<ASTNode>>
}

// an expression on the call stack
    // separate struct because parent pointer is always set after returning from another function
    // so inside a sub-function we can just return part of it and centralise setting of the parent ptr
    // ensuring that a deferredexpr without a parent is invalid and wont go on the stack
pub struct StackExpression {
    pub expr:DeferredExpression,
    pub parent:Option<Rc<ASTNode>>
}

// impl StackExpression {
//     fn get_evaluated(&self)->Option<DataValue>{
//         match &self.expr {
//             DeferredExpr(_) => None,
//             EvaluatedExpr(val) => Some(val.clone())
//         }
//     }

//     fn get_deferred(&self)->Option<DeferredExpression> {
//         match &self.expr {
//             DeferredExpr(de) => Some(de.clone()),
//             EvaluatedExpr(_) => None
//         }
//     }
// }

// body: used for eval, parent: used for checking
#[derive(Clone)]
pub struct DeferredExpression {
    pub ctx:EvalContext,
    pub body:Rc<ASTNode>,
}

// result on the result queue
#[derive(Clone)]
pub struct ExpressionResult {
    pub data:DataValue,
    pub parent:Option<Rc<ASTNode>>
}

pub(crate) fn evaluate_outer(ctx: EvalContext, node: Rc<ASTNode>, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());

    let deferred=DeferredExpression {
        ctx:ctx.clone(),
        body:Rc::clone(&node)
    };

     let stack_expr=StackExpression {
        expr:deferred,
        parent:node.parent.clone()
    };

    evaluate_tco(stack_expr, outer_call)
}

// why does this take EvalContext without ref:
    // because of issue where when returning from UserFunction execute we need a new owned context
    // can't return &Eval since it's created inside the fn body
// Good thing is EvalContext.clone() is cheap because of Rc::clone
use std::collections::VecDeque;


fn resolve(call_stack:&mut VecDeque<StackExpression>, fn_stack:&mut VecDeque<FunctionCall>,results:&mut VecDeque<ExpressionResult>)->Result<()> {
    let expression=call_stack.pop_back().unwrap();

    
    
    Ok(())
}

fn evaluate_tco(expression:StackExpression, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    // println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());
    let mut call_stack: VecDeque<StackExpression> = VecDeque::new();
    let mut fn_stack: VecDeque<FunctionCall> = VecDeque::new();
    let mut results_queue: VecDeque<ExpressionResult> = VecDeque::new();

    let expr_string=&expression.expr.body.to_string();
    call_stack.push_back(expression);

    // what to do with expression on call_st when valid
        // valid: fn_st empty and call_st not empty OR fn_st[-1].ast==call_st[-1].parent
    

    // call_st only: unroll the expr
    // fn_stack only: check res_q
    // both: check fn_st[-1].ast vs call_st[-1].parent
    while !call_stack.is_empty() || !fn_stack.is_empty() {
        let call_has=!call_stack.is_empty();
        let fn_has=!fn_stack.is_empty();

        // both
        if call_has && fn_has {

        }

        // call only
        else if call_has && !fn_has {
            // resolve(&mut call_stack, &mut fn_stack, &mut results_queue)?;
        }
        
        // fn only - fn.execute
        else {

        }
    }

    match results_queue.into_iter().last() {
        Some(res) => {
            Ok(res.data)
        },
        None => {
            let msg=format!("Could not evaluate expression: {}", expr_string);
            return err!("Couldn't evaluate.");
        }
    }
}

