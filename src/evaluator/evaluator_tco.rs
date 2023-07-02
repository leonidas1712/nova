use std::rc::Rc;
use std::result;

use crate::parser::parse_node::*;
use crate::{evaluate_input, lex, message::*, setup_context};

use super::{context::*, data::*, eval_helpers_tco::*, function::*};

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

pub(crate) fn evaluate_outer(ctx: EvalContext,node: Rc<ASTNode>, outer_call: bool,) -> Result<DataValue> {
    // try to match terminals
    println!(
        "Node type: {}, Expr: {}",
        node.get_type(),
        node.to_string_with_parent()
    );

    let deferred = DeferredExpression {
        ctx: ctx.clone(),
        body: Rc::clone(&node),
    };

    let stack_expr = StackExpression {
        expr: deferred,
        parent: node.parent.clone(),
    };

    evaluate_tco(stack_expr, outer_call)
}

// why does this take EvalContext without ref:
// because of issue where when returning from UserFunction execute we need a new owned context
// can't return &Eval since it's created inside the fn body
// Good thing is EvalContext.clone() is cheap because of Rc::clone
use std::collections::VecDeque;

struct ResolveExprArgs<'a> {
    ast:&'a Rc<ASTNode>, // ast for the function call
    children:&'a Vec<Rc<ASTNode>>, // all the children of the expression
    ctx:&'a EvalContext,
    parent:&'a Option<Rc<ASTNode>> // parent of the expression
}

// unroll expression onto call stack and resolve first member to a function then push to fn_stack
fn resolve_expression(call_stack: &mut VecDeque<StackExpression>,fn_stack: &mut VecDeque<FunctionCall>,
    results: &mut VecDeque<ExpressionResult>,args:ResolveExprArgs
)->Result<()> {
    let children=args.children;
    let ctx=args.ctx;
    let parent=args.parent;
    let ast=args.ast;

    // println!("EXPR AST:{} ID:{}", ast.to_string(), ast.original.to_string());

    let ast1_clone=Rc::clone(ast);
    let ast2_clone=Rc::clone(ast);

    // println!("Are the clones equal:{}", ast1_clone.eq(&ast2_clone));
    
    if children.is_empty() {
        return err!("Received empty expression.");
    }

    let first_child = children.first().unwrap();
    let eval_first=evaluate_outer(ctx.clone(), Rc::clone(first_child), false)?;

 
    // we expect first part of expression to resolve to a fn call
    // let and if handled separately already
    let func=eval_first.expect_function()?;
    let func_call=FunctionCall {
        func:func.clone(),
        ast:Rc::clone(ast),
        parent:parent.clone()
    };
    fn_stack.push_back(func_call);

    // push rest of child expressions onto call_st
    let mut rest_children=children.into_iter();
    rest_children.next(); // go past first


    // todo: handle unevaluated separately

    // push in reverse
    for child in rest_children.rev() {
        let deferred=DeferredExpression {
            ctx:ctx.clone(),
            body:Rc::clone(child)
        };
        let stack_expr=StackExpression {
            expr:deferred,
            parent:Some(Rc::clone(ast))
        };

        // assigned parent to one level above supposed to be

        call_stack.push_back(stack_expr);
    }


    Ok(())
}

// check call_st[-1].parent and fn_st[-1].ast
    // ast: pub ast: Rc<ASTNode>,
    // parent: parent: Option<Rc<ASTNode>>,
fn can_resolve(fn_call:&FunctionCall, expr_parent:&Option<Rc<ASTNode>>)->bool {
    let fn_ast=&fn_call.ast;

    match expr_parent {
        Some(parent) => {
            let p=parent.as_ref();
            let fn_a=fn_ast.as_ref();
            let b=p.eq(fn_a);
            b
        },
        None => false
    }
}

// given results queue + func_call -> Vec<Args> 
    // pop from the back of the queue until node with parent!=func.ast
// assume evaluated: uneval handled specially like for if, let
fn get_args<'a>(func:&FunctionCall, results: &'a mut VecDeque<ExpressionResult>)->Vec<Arg<'a>> {
    let mut args:VecDeque<Arg>=VecDeque::new();
    
    // take from back of results queue until we encounter res with diff parent
    for res in results.iter().rev() {
        if !can_resolve(func, &res.parent) {
            break;
        }

        let data=res.data.clone();
        let arg=Arg::Evaluated(data);
        args.push_front(arg);
    }

    // println!("before:{}", results.len());
    // pop after pushing: can't modify during iter
    for i in 0..args.len() {
        results.pop_back();
    }
    // println!("after:{}", results.len());

    args.into_iter().collect()
}


// write function (evaluate_function) to:
    // 1. get arguments from results_q and pop 
    // 2. call func.execute() and get expression
    // 3. if expression is evald: push to res_q
    // 4. else: push to call_st
    // 5. both cases: parent is set to func.parent
fn evaluate_fn(func:&FunctionCall, call_stack: &mut VecDeque<StackExpression>, results: &mut VecDeque<ExpressionResult>)->Result<()>{
    let args=get_args(func, results);
    
    if args.len()==0 {
        let msg=format!("'{}' received 0 arguments.", func.func.to_string());
        return err!("");
    }

    println!("Got args of len:{} for func:{}", args.len(), func.func.to_string());

    Ok(())
}

fn resolve(call_stack: &mut VecDeque<StackExpression>, fn_stack: &mut VecDeque<FunctionCall>,results: &mut VecDeque<ExpressionResult>)
 -> Result<()> {
    let expression = call_stack.pop_back().unwrap();
    let expr = &expression.expr;

    let body = &expr.body;
    let ctx = &expr.ctx;
    let parent=&expression.parent; // dont use body.parent

    let mut result = ExpressionResult {
        data: Num(-1),
        parent: parent.clone(),
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
        }
        _ => {
            todo!()
        }
    }

    Ok(())
}

fn evaluate_tco(expression: StackExpression, outer_call: bool) -> Result<DataValue> {
    println!("EVALUATE START");
    // try to match terminals
    // println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());
    let mut call_stack: VecDeque<StackExpression> = VecDeque::new();
    let mut fn_stack: VecDeque<FunctionCall> = VecDeque::new();
    let mut results_queue: VecDeque<ExpressionResult> = VecDeque::new();

    let expr_string = &expression.expr.body.to_string();
    call_stack.push_back(expression);

    // what to do with expression on call_st when valid
    // valid: fn_st empty and call_st not empty OR fn_st[-1].ast==call_st[-1].parent

    // call_st only: unroll the expr
    // fn_stack only: check res_q
    // both: check fn_st[-1].ast vs call_st[-1].parent
    while !call_stack.is_empty() || !fn_stack.is_empty() {
        let call_has = !call_stack.is_empty();
        let fn_has = !fn_stack.is_empty();

        // both - check ast vs parent
        if call_has && fn_has {
            let call_st_last=call_stack.back().unwrap();
            let fn_st_last=fn_stack.back().unwrap();
            
            if can_resolve(fn_st_last, &call_st_last.parent) {
                resolve(&mut call_stack, &mut fn_stack, &mut results_queue)?;
            
            // when call_stack[-1] doesnt match fn_st[-1]: evaluate
            } else {
                evaluate_fn(fn_st_last, &mut call_stack, &mut results_queue)?;
                break;
            }
        }

        // call only
        else if call_has && !fn_has {
            resolve(&mut call_stack, &mut fn_stack, &mut results_queue)?;
        }
        // fn only - fn.execute
            // 1. get correct args from result queue
            // 2. pass to fn execute, get Expression
            // 3. push onto res_q with correct parent=fn_ast.parent

        else {
            let func=fn_stack.back().unwrap();
            evaluate_fn(func, &mut call_stack, &mut results_queue)?;
            break;
        }
    }

    match results_queue.into_iter().last() {
        Some(res) => Ok(res.data),
        None => {
            let msg = format!("Could not evaluate expression: {}", expr_string);
            return err!("Couldn't evaluate.");
        }
    }
}
