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

pub enum Expression {
    Deferred(DeferredExpression),
    Result(EvaluatedExpression)
}

// body: used for eval, parent: used for checking
pub struct DeferredExpression {
    ctx:Rc<Context>,
    body:Rc<ASTNode>,
}

// this will get transferred to the result queue
pub struct EvaluatedExpression {
    data:DataValue,
}

// why does this take EvalContext without ref:
    // because of issue where when returning from UserFunction execute we need a new owned context
    // can't return &Eval since it's created inside the fn body
// Good thing is EvalContext.clone() is cheap because of Rc::clone
pub(crate) fn evaluate_tco(ctx: EvalContext, node: Rc<ASTNode>, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());
  

    match &node.value {
        Boolean(b) => Ok(Bool(*b)),
        Number(num) => Ok(Num(*num)),
        Symbol(sym) => {
            // Function
            let read=ctx.read();
            let fnc = read.get_function(sym);
            if fnc.is_some() {
                let cloned = fnc.unwrap().clone();
                return Ok(FunctionVariable(cloned));
            }

            // Variable
            let resolve = read.get_variable(sym);
            if resolve.is_some() {
                Ok(resolve.unwrap().clone())
            } else {
                let err_string = format!("Unrecognised symbol: '{}'", sym);
                err!(err_string.as_str())
            }
        }
        List(children) => evaluate_list(&ctx, children),
        IfNode(children) => {
            return evaluate_if(
                &ctx,
                children.get(0).unwrap(),
                children.get(1).unwrap(),
                children.get(2).unwrap(),
            );
        },
        LetNode(children, global) => evaluate_let(&ctx, children, *global),
        FnNode(fn_def) => evaluate_fn_node(&ctx, fn_def, outer_call),
        Expression(children) => evaluate_expression(&ctx, children),
    }
}

