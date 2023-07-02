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

pub(crate) fn evaluate(ctx: EvalContext, node: Rc<ASTNode>, outer_call: bool) -> Result<DataValue> {
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
        List(children) => evaluate_list(ctx, children),
        IfNode(children) => {
            return evaluate_if(
                ctx,
                children.get(0).unwrap(),
                children.get(1).unwrap(),
                children.get(2).unwrap(),
            );
        },
        LetNode(children, global) => evaluate_let(ctx, children, *global),
        FnNode(fn_def) => evaluate_fn_node(ctx, fn_def, outer_call),
        Expression(children) => evaluate_expression(&ctx, children),
    }
}

// to call with default false outer_call
#[macro_export]
macro_rules! eval {
    ($ctx:expr, $node:expr) => {
        crate::evaluate($ctx, $node, false)
    };
}

pub(crate) use eval;

use crate::lexer::Lexer;
use crate::parser::parser::parse;

pub fn test_eval(expr: &str, expected: &str) {
    let l = lex!(expr);
    let p = parse(l).unwrap();
    let ctx = EvalContext::new();
    let e = evaluate(ctx, p, true).unwrap().to_string();

    assert_eq!(e, expected);
}

pub fn test_eval_many(exprs: Vec<&str>, expected: Vec<&str>) {
    for tup in exprs.into_iter().zip(expected.into_iter()) {
        test_eval(tup.0, tup.1);
    }
}

#[test]
fn let_test() {
    let exprs = vec![
        "(let x 2)",
        "(let x 3 y 4 (add x y))",
        "let x 3 y 4 (add x y)",
        "let x 3 y (let x 10 x) (add x y) ",
    ];

    let exp = vec!["2", "7", "7", "13"];

    test_eval("(let x 2)", "2");
}
