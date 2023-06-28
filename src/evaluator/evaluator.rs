use super::{context::*, data::*};
use crate::message::*;
use crate::parser::node::*;

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

// worst case: borrowing from context e.g if we return a FunctionVariable it shouldnt last longer than the context
pub fn evaluate<'a>(ctx: &'a Context, node: &ASTNode) -> Result<DataValue> {
    let mut nova_result = NovaResult::new(DataValue::Default);

    // try to match terminals
    match &node.value {
        Number(num) => nova_result.result = Num(*num),
        Symbol(sym) => {

        }
        _ => nova_result.result = DataValue::Default,
    }

    Ok(nova_result)
}
