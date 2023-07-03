use std::char::MAX;
use std::cmp::max_by;
use std::rc::Rc;

use crate::parser::parse_node::*;
use crate::{evaluate_input, lex, message::*, setup_context};

use super::eval_helpers::*;
use super::{context::*, data::*, function::*};

use std::cell::RefCell;
thread_local! {
    pub (crate) static DEPTH: RefCell<u64> = RefCell::new(0);
    pub (crate) static MAX_DEPTH:RefCell<u64>=RefCell::new(0);
}

pub (crate) fn update_depth() {
    DEPTH.with(|x| {
        let mut rf = x.borrow_mut();
        let val = *rf;
        *rf = val + 1;

        MAX_DEPTH.with(|r| {
            let mut max_depth = r.borrow_mut();
            let max_value = *max_depth;
            *max_depth = max_value.max(val + 1);

            // println!("Max depth: {}", *max_depth);
        });
    });
}

pub (crate) fn subtract_depth() {
    DEPTH.with(|x| {
        let mut rf = x.borrow_mut();
        let val = *rf;
        *rf = val - 1;
        // println!("Subtracted depth:{}", *rf);
    });
}

pub (crate) fn print_max_depth() {
    MAX_DEPTH.with(|x|{
        let mut rf=x.borrow();
        println!("Max depth:{}", *rf);
    });
}
// why does this take EvalContext without ref:
// because of issue where when returning from UserFunction execute we need a new owned context
// can't return &Eval since it's created inside the fn body
// Good thing is EvalContext.clone() is cheap because of Rc::clone
pub(crate) fn evaluate(ctx: EvalContext, node: Rc<ASTNode>, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    // println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());

    // update_depth();

    let result = match &node.value {
        Boolean(b) => Ok(Bool(*b)),
        Number(num) => Ok(Num(*num)),
        Symbol(sym) => {
            // Function
            let read = ctx.read();
            let fnc = read.get_function(sym);
            if fnc.is_some() {
                let cloned = fnc.unwrap().clone();
                Ok(FunctionVariable(cloned))
            } else {
                // Variable
                let resolve = read.get_variable(sym);
                if resolve.is_some() {
                    Ok(resolve.unwrap().clone())
                } else {
                    let err_string = format!("Unrecognised symbol: '{}'", sym);
                    return err!(err_string.as_str());
                }
            }
        }
        List(children) => evaluate_list(&ctx, children),
        IfNode(children) => evaluate_if(
            &ctx,
            children.get(0).unwrap(),
            children.get(1).unwrap(),
            children.get(2).unwrap(),
        ),
        LetNode(children, global) => evaluate_let(&ctx, children, *global),
        FnNode(fn_def) => evaluate_fn_node(&ctx, fn_def, outer_call),
        ParseExpression(children) => evaluate_expression(&ctx, children),
    };

    // subtract_depth();
    result
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

// #[test]
// fn depth_test() {
//     let recr="(def recr (n) (if (eq n 0) 0 (recr (pred n))))";
//     let ctx=EvalContext::new();

//     let l=lex!(recr);
//     let p=parse(l);

// }
