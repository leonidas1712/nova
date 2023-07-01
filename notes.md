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

call_st: has expressions
-> could be terminal or non-terminal
           
Expression(DataValue, &ctx, body:&astnode, ast:&astnode) - &DataValue?
-> separate body from ast -> ast to use for checking parent etc, body to use for eval
    -> e.g (fn x) -> (succ x) -> eval use (succ x), checking use (fn x)
    -> expr unrolling: use actual body as parent

FnCall: need to set parent of ret expr properly
-> eval: use parent of fn symbol inside expr

FnCall(Rc<dyn Function>, ctx, body, ast) -> possibly just the Rc
Result(DataValue,ast)

Expression struct: ctx,body, ast
-> data: enum
    -> FnCall or Data

    
   


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


