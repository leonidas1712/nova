Input: string from user
-> tokenize by replacing all keywords with spaces
-> split
-> return iterator (Lexer)

A borrower should never outlive the borrowed data

Make a repl to take input, do something, output

combine Function into one trait, arguments can be an enum -> separate eval/uneval

struct for data types:?
-> Data struct, contains enum for value

struct Data {
    value: DataValue
}

enum DataValue {
    Number(u32),
    List(PairStruct),
    Boolean(bool),
    FunctionVariable(dyn),
        -> FunctionVariable: stores a dyn trait Function    
}

struct Context {
    identifiers: HashMap key=>data (?)
    functions: HashMap key=> dyn Function object
}

trait ExecutableFunction {
    just call evaluate directly
    execute(res:Vec<Args>, ctx:Context, //evaluate:?) {...}
    -> 1. method to unwrap args to eval/uneval then check the types easily without match
        -> get right number of eval/unevaluated 
        -> inf: check all eval/ all uneval
    -> 2. unwrap the data value inside for Evaluated
        -> e.g expect Number, expect List, etc
        -> infinite: should be able to expect for the whole list -> unwrap and get the ints if 
            all possible
}

1. how to do higher order functions: where functions can also be a variable
-> use enum in Data: HigherOrderFunction(...)
    -> hof contains a reference to the actual function
2. how to do currying -> both function and builtin need to have currying
-> each struct has the list of args-> no choice but to repeat?
    -> or maybe use default method on Function trait

the only difference is builtin doesnt have a function body

Just use trait Function, then write the builtin functions manually as structs impl Function
then when you want to use just add them all to the hashmap: now builtins and user function are treated the exact same

// references to functions: don't want to clone everything everytime we copy a ctx and pass to a new expression
// just copy the refs
// same for variables/identifiers: just copy
// currying: always make a new function, don't need to mutate the old one (in fact we shouldn't mutate)
// currying new context: also make a new context, but with new identifier added
// copy all the old references into new ctx
// identifiers: string => &DataValue

// why separate maps: because to check if something is a function dont want to search the entire namespace
// and also we want same namespace for functions and variables, otherwise i could just have a set of function names

// functions: map (String => &Box<dyn Function>)
// identifiers: map

// idea:
// variables: map (String => &DataValue)
// include FunctionVariables
// function_names: Set(String)
// maintain invariant for shared namespace(?) e.g x=2, x=function, get x => should be a function

// maintain a pointer to the previous context
// copy: pass the pointer down

// Context problem: how to copy contexts from one expression to a child expression
// e.g first ctx: y=4, map=some function. second ctx to eval child expr: defines y=5, but should have access to map
// child contexts should be able to overwrite but only within themselves not in the parents
// 1. maintain pointers to parents
// expensive: if we recurse down 1000 levels we need to search up to 1000 levels for a variable
// 2. clone all the data each time
// the most flexible solution, but would be good to not have to copy data like lists and functions
// e.g a list with 10000 elements in context 1, copied to context 2 -> expensive

// Function evaluation: needs to store context at time of creation, then arguments
// needs to clone parent context: if we use a reference any changes would be reflected => can't have closure
// but then if Context needs to copy by cloning -> Context needs to clone
// but Ctx has DataValue -> does it need to deepcopy or just copying enums?

// Function: has a inner Context (cloned, owns)
// Context: has DataValue, if it has a function it has only a reference to the function

// we dont want to deepcopy everything, but copying references is ok
// clone: make a new hashmap, copy the strings, then copy the references to data (normal copy)
// but if the map is of names to references, then when we add something it won't live long enough
// since it will be dropped when that function exits
// if the map is of names to owned data -> then we need to deepcopy

// do we ever need to mutate functions => No
// builtin: defined once, currying -> copy
// user: defined once, currying -> copy

// 1. function name set + map of names->DataValue
// 2. how to convert Args

Args {
    Evaluated(DataValue),
    Unevaluated(ASTNode)
}

struct UserFunction {
    put name, args, fn body, inner context here
    when function is applied: can curry by updating arg idx
    ...
}

struct BuiltInFunction {
    function: <ptr to function>
    -> function: (Vec<Args>, Context, evaluate)
        -> evaluate: (Context, ASTNode)
}

impl ExecutableFunction for UserFunction {
    -> access args, fn body...
    execute(...) => err on Args::Unevaluated
}

impl ExecutableFunction for BuiltInFunction {
    execute(...) => specific builtin can specify err for args
}




#[derive(Debug)]
enum Args {
    Evaluated(DataValue),
    Unevaluated(String)
}

impl Args {
    fn get_data_value(self)->Option<DataValue> {
        match self {
            Args::Evaluated(val) => Some(val),
            _ => None
        }
    }
}

#[derive(Debug)]
struct Boolean {
    value:bool
}

#[derive(Debug)]
struct Number {
    value:usize
}

#[derive(Debug)]
enum DataValue {
    Bool(Boolean),
    Num(Number)
}

impl DataValue {
    fn get_boolean(self)->Option<Boolean> {
        match self {
            DataValue::Bool(b) => Some(b),
            _ => None
        }
    }
    
    fn get_num(self)->Option<Number> {
        match self {
            DataValue::Num(num) => Some(num),
            _ => None
        }
    }
}

fn main() {
    let number=Number{value:30};
    let boolean=Boolean{value:true};
    
    let bool_enum=DataValue::Bool(boolean);
    let num_enum=DataValue::Num(number);
        
    let arg1=Args::Evaluated(bool_enum);
    let arg2=Args::Evaluated(num_enum);
    let arg3=Args::Unevaluated("some ast".to_string());
    
    let args=vec![arg1,arg2,arg3];
    
    let evaled:Vec<Option<DataValue>>=args.into_iter().map(|x| x.get_data_value()).collect();
    
    let ok=evaled.iter().map(|x| x.is_some()).all(|x| x==true);
    
    dbg!(ok);

}

Map of string => DataValue
+ set of function names
-> we need to keep set of var names and set of fn names disjoint

assumption: set is already disjoint (base case: both empty, true)
-> ident: x, new value: new
    -> 1. currently a variable:
        -> a. new is var: replace in map => still disjoint
        -> b. new is function: repl, add to fn set => still disjoint 
    -> 2. currently a function:
        -> a. new is var: remove from fn set, repl in map -> remove from fn + add to var => disj
        -> b. new is function: remove fron fn set, repl in map => set was already disjoint so ident is not in vars

Rc vs Box
https://stackoverflow.com/questions/49377231/when-to-use-rc-vs-box



- dont eval until arg parent matches fn_st parent
Push onto stack:
- ASTNode x original s

add rule: 
- first keyword should be 'let' OR a function call
    - otherwise invalid

e.g (2 2 3) -> invalid
(x 2 3) where x is not a function -> invalid

0. append expr in reverse to call_st (only if condition1 satisfied)
1. call_ast[-1].parent == fn_ast[-1].parent : eval call[-1], append to res_q
2. != : inter=prepend from back of res
3. when promoting: 
-> if result is a function and parent is marked as fn call: push to function stack with new ast
-> else, push to result
    -> this way we can keep evaluating when res is a function

( sum ((map fn) lst)  x)-> what happens when need to eval (map fn) to get func var
    -> still works as long as we promote properly: (map fn) result is promoted
    -> but this wont work completely for lambdas until we add @ parent
        // ex: (g->x->g)(1)(2)

        -> because e.g f->x->y will receive 2 arguments instead of 1
            -> 
 
TCO:
- three structures:
    - exprs stack - Expression(&ctx, &ast node)

    - fn stack: Expression(&ctx, &ast node)
        - only evaluate when needed not eagerly - to make parent checking easier
            - e.g (map fn) -> we can check (map fn) ast.parent directly

    - results queue - ResultNode(&DataValue,&ast)
        -> &ast represents result node ast for parent checking
        
 Mark ast nodes during parsing as function calls or not: based on is it the first node in expr
        -> during promotion we simply look at the mark to decide if function result goes to
            fn_stack or res_queue
 -> then when we check parent ast it will be accurate (with the correct parent node)
                in case the function variable comes from another expression e.g If, Let, Expr..
            

TCO algorithm:
init: call_st, fn_st, res_q
Before: add initial expression to call_st
0. Pop from call_st
1. Resolve terminals first -> everything but expression
    -> currently: recursion for fn defs and let
2. Expression: (brackets)
    a. Add children in reverse to call_st
    b. Index 0: add to function variable stack -> don't evaluate until needed

while call_st or fn_st:
3. Now check call_st[-1] and fn_st[-1]:
    a. if they share the same parent: resolve call_st[-1], push to res_q
    b. (call_st is empty or) else: f=fn_st[-1]
            -> if f is not a function variable: decompose f and push to call_st, continue
                -> decomp: if its an expression unpack it here, dont push to call_st first
                    -> otherwise we have below issue
                
                -> issue here with (sum ((map fn) 1) ) ***
                    -> when you want to push (map fn) onto f, sum and (map fn) prt dont match
                    -> solve by either using @ to separate expressions or eval fn eagerly

        b.1 args = go from the back of res_q, prepend until we reach a res that is not same parent
            -> then evaluate f(args) and append to res_q
                -> when appending: 'promote' f(args) so its parent becomes f.parent.parent
4. return res_q[-1]

prepend until reach parent not same
and call_st: check parent to see if we need to eval
when f_st[-1] and call_st[-1] same parent but call_st[-1] is a expr: unroll the expr, dont eval it
    -> but otherwise we can try to resolve directly

first: eagerly evaluate function variables (the first exp)
    -> once @ is added: can delay eval

two cases for fn eval:
1. when call_st becomes empty
2. when call_st[-1] and fn_st[-1] not same parent
    -> then we go through res_q from back, prepend, etc.

returning from a fn:
-> if res ast marked as fn: put on fn_st
-> else: put on call_st 
    -> includes both deferred exprs and resolved data values

call_st: has expressions
-> could be terminal or non-terminal

***
Expression(DataValue, &ctx, body:&astnode, ast:&astnode) - &DataValue?
-> separate body from ast -> ast to use for checking parent etc, body to use for eval
    -> e.g (fn x) -> (succ x) -> eval use (succ x), checking use (fn x)
    -> expr unrolling: use actual body as parent

***
FnCall: need to set parent of ret expr properly
-> eval: use parent of fn symbol inside expr
when flattening to fn call: set fn call ast to the parent of the symbol 
e.g (id 1) -> id's ast is (id 1), not id
-> then comparison is fn_st[-1].ast==call_st[-1].parent
-> eval(f_st[-1], res) => return expr.parent=f_st[-1].ast


FnCall(Rc<dyn Function>, ctx, body, Rc<ast>) 
Result(DataValue,Rc<parent>) 

Expression struct: ctx,body,ast (replace with parent?)
-> ast used for cmp
-> data: enum
    -> FnCall or Data

issue if clones considered equal: because when evaluating function bodies need to clone nodes(?)

**1.Expr(&ctx, &body_ast, &parent_ast)  -> body and parent separate -> body.parent not always parent
-> unrolling an expr: 
    -> all children going on call_st (i.e all except first) have parent set to unrolled expr
    -> fn_call expr: has ast set to unrolled expr
-> Expr: two types
    -> Deferred(ctx, body, parent)
    -> Evaluated(data_value, parent) -> when we see this we can transfer it to a result
        -> this is a subtype of expr so that we can make all functions return a general Expression
            -> even though its basically the same as 'Result'
        -> this helps us implement currying: so that functions with < req args can return curried fn
-> ctx: Rc<Context>
    -> when unrolling: Rc::clone
    -> return from function: make Rc of merged context -> transferred ownership 
-> body_ast:Rc<ASTNode>
    -> return from function: clone the node and make into new Rc (clones unequal)
-> parent_ast: Rc<ASTNode>
    -> use Rc::clone when unrolling to set parent

-> comparison: fn_st[-1].ast == call_st[-1].parent
    -> equals: compare by some id

**2.FnCall(&Rc<dyn Function>, &ast, &parent) 
- ast used for comparison with expr.parent_ast
- parent used for which ast to promote result to (result_expr's parent becomes this parent)
    -> FnCall: need to set parent of ret expr properly
    -> eval: use parent of fn symbol inside expr
    -> may need to store extra &parent -> yes
    -> need to clone fn_body when returning out as expression so that parent can be set
        -> original body's parent should remain as None since it changes with every call

    when flattening to fn call: set fn call ast to the parent of the symbol 
    e.g (id 1) -> id's ast is (id 1), not id
    -> then comparison is fn_st[-1].ast==call_st[-1].parent
    -> eval(f_st[-1], res) => return expr.parent=f_st[-1].ast.parent**
        -> i.e promote to f_st[-1].ast.parent
        -> store parent ref in FnCall as well to avoid cloning

**3.ExprResult(DataValue, &parent)
    -> transfer from call_st to res_q: just use parent pt

resolve function: when parents match, what to do
    -> expr: unroll it -> parents of exprs on call_st = expr, fn_call.ast is expr,
        -> and fn_call.parent is expr.parent
    -> everything else: resolve to ExprResult, push to res_q
        -> return value: Result<Option<ExprResult>>
    
handle if specially: IfNode => evaluate to Expression with the correct branch
    -> resolve if: returned Expr parent is set to IfNode's parent
let and fn_def: how to handle ctx return

**4.FnCall:
-> for now: evaluate the first term recursively and see if we get a function out
-> returned expression: make the parent = fn_st[-1].ast.parent (promotion step)
-> if returned is a function and promoted ast is marked as fn_call: need to add to fn_stack instead

General rule:
-> when eval fn_st[-1]: the returned value parent is set to fn_st[-1].parent

return if: expr parent set to ifnode parent
return fndef: returned
-----
if call_st empty and fn_st empty: unroll expr by default
-> maybe first time unroll

call_st and fn_st: then check call[-1] and fn[-1] prt match
    -> match: resolve call[-1] (if expr, means unroll)
    -> else: eval using fn[-1] and call[-1]

call_st empty but fn_st: apply fn[-1] to results from res[-1]..0 (where prts match)

eval of a function:
-> set ast of returned expr to fn ast's parent
e.g (succ 5) => 6's ast becomes (succ 5)
(sum x) -> (add 2 (succ x))
-> eval (sum 5) => (add 2 (succ x)) ast becomes (sum 5) not just sum



eval function: return expr node needs to be cloned uniquely
e.g (recr 2)'s body should be different from (recr 1)'s body even though its the same if node
-> in fact (recr 1) may be different from (recr 1) e.g fn(a,b,c)=>a+b+c

(fn 1 (fn 1 2 0) 2) => 6



places that may cause real left recursion:
-> resolution of functionvariable
-> resolution of if cond result e.g  if (recr n-1) ...





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

---

 cases:
 - function is curried 
 - lambdas
 - other nestings
 - if exprs
 - let - context - how to manage ctx

- possibly: eliminate ctx copying by using a ctx stack + bin search
    -> how would it work for functions with closures? - maybe only this case needs copying,
        -> then it just becomes another ctx to add?

when do we copy ctx:
- let exprs
    - can be done iteratively by reversing the expression and maintaining an ident stack + ctx stack (?)
- fn curry


<!-- 1. add comments -->
2. how to handle function currying without binding every space
    -> go through args. eval fn(a)
        -> if result is a function set fn=res, then keep going with a2..
        -> at the end: return the result
    -> for inf args: need to check if parent ast is marked func_call when promoting result 
        -> if yes: return curried fn, else return result
    
curry:
    - number of args: finite or inf
    - finite:
        - apply args one at a time (or use num args to pass in)
        - once args reached: eval -> ret DataValue
            - if ret value is a function: use this new function to keep evaluating
    - curry method:
        - take Vec<Args>, finite: arg names + ctx, inf: args array
        - always return curried function
    - outside: func.execute() => check parent
        - if parent is fncall: return function
        - else: func.eval() -> result (Result<DataValue>)
<!-- 3. :del and others -->
<!-- 4. unit type -->


5. splitting by regex

// break when returned is not a function or args run out
    // ret: 1. DE:return out, put on call_st. 2. Data:Var - ret out,
    // 3. Data:Fn
        // a. have more args: continue eval
        // b. no more args: ret out

    // return is deferred expr?: put on call stack with sa
        // 1. take args according to number (inf: all, finite: n)
        // 2. pass to execute, get result
        // 3. if expr is not a func or args left 0: return out.
        // 4. else: func=res, continue

(def fn (a,b) (add a b))

func_call, args:Vec<Arg>
# (fn 1 2 3 4)  - currently err #
# ((add 1 2) 3 4) - curr err #

-> func: Finite or Inf
->  finite(n)
    -> args > n : (fn 1 2 3 4...) 
        -> ret value another func: ok, continue with func=result, num_args=func.num_args
        -> else: err
    args <= n: just return out 
    -> args == n: (fn 1 2) => data
        -> ret value: whatever it is return out
    -> args < n: (fn 1) => curried
        -> ret value: whatever it is return out

-> inf
    -> initially: just evaluate no currying
    -> args < min: (add 1) => err
    -> args == min: (add 1 2) => check parent if is_func or not
    -> args > min: (add 1 2 3 ..) => check parent if is_func or not
        -> is_func True => return curried fn
        -> false => return result (coerce)



// 1. if func.ast is_func (result expected to be function) 
    // call func.execute -> should get a FunctionVariable
    // put on fnstack with ast=parent, parent=parent.parent

    execute: 
        -> finite: err if args > expected. else, return curried function
            -> clone Rc<ctx>
        -> inf: err if args < min. else, return curried function
            -> clone vec first, later traverse up LL

// 2. if func.ast NOT is_func (result expected to be final result) -> call func.resolve
    // put on call_stack or result as normal        

Uneval/Eval, Finite/Inf

Finite/inf

1. finite
    -> curry as long as params avail
2. inf
    -> always return a curried function
    -> caller: check parent to know if must resolve


0. params
1. state for builtins (params)
2. execute/resolve

// resolve: Result<Expression>
    // Finite: if args < expected, return curried function. == exp: return value. > exp: err
    // inf: args < min: curried. else: return result

get_args -> fin -> fin.num_exp 
 => 0 means == exp: call execute




trait Function
-> get_curr_args(&self)->&[Arg]
    -> use for resolve
-> get_arity(&self) -> NumArgs : abstract
    -> actual: use Params.get...
-> apply(args:&[Arg])->Result<Rc<Function>> : abstract
    -> actual: use Params.apply
-> resolve(&self) -> Result<Expression>

// what to do when all args are received
-> execute(args:&[Arg], ctx:EvalContext)->Result<Expression> : abstract
    -> actual: 
        UserFn -> same as before
        BI->call the function


trait Function2
-> get_curr_args(&self)->&Params
    -> use for resolve
-> get_arity(&self) -> NumArgs : abstract
    -> actual: use Params.get...
-> apply(args:&[Arg])->Result<Rc<Function>> : abstract
    -> actual: use Params.apply
-> resolve(&self, ctx:&EvalContext) -> Result<Expression>

// what to do when all args are received
-> execute(args:&[Arg], ctx:&EvalContext)->Result<Expression> : abstract
    -> actual: 
        UserFn -> same as before
        BI->call the function


BuiltIn
    -> members: params, name, function to use for eval
        -> fn: &[Arg], ctx -> Result<Expression>
    -> get_curr, get_arity, apply
    -> evaluate: call fn.eval

User:
    -> same as before

pub fn add...
BI::new(params::Infinite, "add", add_fn)

pub fn cons...
BI::new(params::Finite("head", "tail"), "cons", cons_fn)

let succ=BI::new(params::Finite("num"), "succ", succ_fn)
succ.apply(1,2,3) 
    -> params.apply -> check for too many

succ.resolve()
    -> check args 


let add=BI::new(params::Infinite, "add", add_fn)
    -> add.apply(1,2,3)...
    -> add.resolve() -> check params arity matches here





    

// UserFn: name, ctx, body, params
    // resolve: call fn body,merge, etc.
// BuiltIn: name, params, <resolve fn>

// BuiltIn(Add, Infinite)
    // execute(...)

// BuiltIn(Cons, Finite)
    // execute(...)
