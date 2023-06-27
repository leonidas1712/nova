use super::{context::*, data::*};

use crate::parser::*;
use crate::parser::node::*;
use crate::message::*;

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
        // i.e if first element resolves to FunctionVariable + first element +...etc
    // else, evaluate the other subexpressions in order and return the result from the last eval
        // e.g (puts 1) (puts 2 ) (puts 3) => should print 1 2 3
    
// Else: invalid, return error

// worst case: borrowing from context e.g if we return a FunctionVariable it shouldnt last longer than the context
pub fn evaluate<'a>(ctx:&'a Context, node:&ASTNode, user_fn:bool)->Result<DataValue<'a>> {
    let mut nova_result=NovaResult::new(DataValue::Default);

    if user_fn {
        println!("Called by user function:{}", node.value.to_string());
        return Ok(nova_result)
    }
    // placeholder
    

    println!("eval");
    context();

    Ok(nova_result)
}