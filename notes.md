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
    execute(res:Vec<Args>, ctx:Context, evaluate:?) {...}
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