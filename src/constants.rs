// Tokens
pub const OPEN_EXPR: &str = "(";
pub const CLOSE_EXPR: &str = ")";
pub const NEWLINE: &str = "\n";
pub const TAB: &str = "\t";
pub const VAR_SEP: &str = ",";
pub const OPEN_LIST: &str = "[";
pub const CLOSE_LIST: &str = "]";
pub const SPACE: &str = " ";
pub const EMPTY: &str = "\0";

// Keywords
pub const LET: &str = "let";

// Operations
pub const ADD: &str = "add";
pub const MULT: &str = "mul";
pub const SUB: &str = "sub";
pub const DBL: &str = "dbl";
pub const INC: &str = "succ";
pub const DEC: &str = "pred";
pub const EQUALS: &str = "eq";
pub const FN: &str = "def";
pub const IF: &str = "if";
pub const PUTS: &str = "puts";
pub const PRINT: &str = "prt";
pub const OR: &str = "or";
pub const AND: &str = "and";
pub const IMPORT: &str = "import";
pub const CHAIN: &str = ">";
pub const SET: &str = "set";
pub const GET: &str = "get";
pub const LT: &str = "lt";
pub const GT: &str = "gt";
pub const MOD: &str = "mod";
pub const DIV: &str = "div";

// List ops
pub const CONS: &str = "cons";
pub const CAR: &str = "car";
pub const CDR: &str = "cdr";
pub const LCONS: &str = "lcons";
pub const LCDR: &str = "lcdr";
pub const LCAR: &str = "lcar";
pub const INDEX: &str = "idx";


// Lambda
pub const LAMBDA: &str = "->";
pub const LAMBDA_TYPE: &str = "lambda";

// Binary operations
pub const COMP_OPR: &str = "$";
pub const COMP_LEFT: &str = "@";
pub const PIPE: &str = ">>";


// Tokens to split by
pub const SPLIT_TOKENS: [&'static str; 11] = [
    OPEN_EXPR, CLOSE_EXPR, NEWLINE, TAB, VAR_SEP, 
    OPEN_LIST, CLOSE_LIST, SPACE, LAMBDA, COMP_OPR, PIPE
];

pub const DONT_ADD:[&'static str;5]=[NEWLINE, TAB, VAR_SEP, SPACE,EMPTY];
