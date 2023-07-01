use super::eval_helpers::*;
use super::{context::*, data::*};
use crate::parser::parse_node::*;
use crate::{evaluate_input, lex, message::*, setup_context};

// 1. Check ast node type -> if terminal, convert to a DataValue -> put this method in context
// terminal: num, bool, list, function variable, identifiers
// identifiers: look at context
// num,bool,list: can do directly

// 2. non-terminals but handled differently: IfStmt, LetStmt, FnDef
// if node has that type => pass to those methods for eval (handle_if, handle_let, resolve_function)

// 3. Covered: Number, List, IfStmt, LetStmt, FnDef

// 4. Left with expression
// Resolve based on first element: if first element resolves to function call (first could also be an expression), call the function with
// the rest of the expressions as arguments
// We are in an expression + First subexpr resolves to FunctionVariable + length of expr > 1 => eval

// FunctionCall: check if evaluated or unevaluated, then decide to eval or not the rest of the subexprs
// Need a way to check - trait
// else, evaluate the other subexpressions in order and return the result from the last eval
// e.g (puts 1) (puts 2 ) (puts 3) => should print 1 2 3

// Else: invalid, return error

// how to handle: ( (def f (x) x) (1) )
// i.e inline fn def + result
// make DataValue::FnDef -> contains Rc<fn> + optional result
// return out -> add to REPL context
// lambda: can just return a normal FnVar

// default to false

pub(crate) fn evaluate(ctx: &Context, node: &ASTNode, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    // println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());

   
    match &node.value {
        Boolean(b) => Ok(Bool(*b)),
        Number(num) => Ok(Num(*num)),
        Symbol(sym) => {
            // Function
            let fnc = ctx.get_function(sym);
            if fnc.is_some() {
                let cloned = fnc.unwrap().clone();
                return Ok(FunctionVariable(cloned));
            }

            // Variable
            let resolve = ctx.get_variable(sym);
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
        }
        LetNode(children, global) => return evaluate_let(ctx, children, *global),
        FnNode(fn_def) => return evaluate_fn_node(&ctx, fn_def, outer_call),
        Expression(children) => evaluate_expression(ctx, children),
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
    let ctx = setup_context();
    let e = evaluate(&ctx, &p, true).unwrap().to_string();

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
