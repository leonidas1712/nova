use std::rc::Rc;

use crate::parser::parse_node::*;
use crate::{evaluate_input, lex, message::*, setup_context};

use super::eval_helpers::*;
use super::{context::*, data::*, function::*};

pub struct FunctionCall {
    func:Rc<dyn Function>,
    ast:Rc<ASTNode>,
    parent:Option<Rc<ASTNode>>
}

// an expression on the call stack
    // separate struct because parent pointer is always set after returning from another function
    // so inside a sub-function we can just return part of it and centralise setting of the parent ptr
pub struct StackExpression {
    expr:Expression,
    parent:Option<Rc<ASTNode>>
}

#[derive(Display)]
pub enum Expression {
    Deferred(DeferredExpression),
    Result(EvaluatedExpression)
}

// body: used for eval, parent: used for checking
pub struct DeferredExpression {
    ctx:EvalContext,
    body:Rc<ASTNode>,
}

// this will get transferred to the result queue
pub struct EvaluatedExpression {
    data:DataValue,
}

pub struct ExpressionResult {
    data:DataValue,
    parent:Option<Rc<ASTNode>>
}

pub(crate) fn evaluate_outer(ctx: EvalContext, node: Rc<ASTNode>, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());

    let deferred=DeferredExpression {
        ctx:ctx.clone(),
        body:Rc::clone(&node)
    };
    
    let stack_expr=StackExpression {
        expr:Expression::Deferred(deferred),
        parent:node.parent.clone()
    };

    let res=evaluate_tco(stack_expr, outer_call);

    Ok(Default)
}

// why does this take EvalContext without ref:
    // because of issue where when returning from UserFunction execute we need a new owned context
    // can't return &Eval since it's created inside the fn body
// Good thing is EvalContext.clone() is cheap because of Rc::clone
use std::collections::VecDeque;

pub(crate) fn evaluate_tco(expression:StackExpression, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    // println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());
    let call_stack: VecDeque<StackExpression> = VecDeque::new();
    let fn_stack: VecDeque<FunctionCall> = VecDeque::new();
    let results_queue: VecDeque<ExpressionResult> = VecDeque::new();

    Ok(Default)
}

