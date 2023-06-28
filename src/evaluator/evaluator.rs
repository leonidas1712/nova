use super::{context::*, data::*};
use crate::message::*;
use crate::parser::node::*;
use super::eval_helpers::*;

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

pub (crate) fn evaluate(ctx:&Context, node: &ASTNode) -> Result<DataValue> {
    // try to match terminals
    match &node.value {
        Boolean(b) => Ok(Bool(*b)),
        Number(num) => Ok(Num(*num)),
        Symbol(sym) => {
            // Boolean

            let fnc=ctx.get_function(sym);
            if fnc.is_some() {
                return Ok(FunctionVariable(fnc.unwrap().clone()));
            }

            let resolve=ctx.get_variable(sym);
            if resolve.is_some() {
                Ok(resolve.unwrap().clone())
            } else {
                let err_string=format!("Unrecognised symbol: '{}'", sym);
                Err(Ex::new(err_string.as_str()))
            }
        },
        Expression(children) => evaluate_expression(ctx, children),
        List(children) => evaluate_list(ctx, children),
    }
}
