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

pub type NumType = i64;

// Keywords
pub const LET_NAME: &str = "let";
pub const IF_NAME: &str = "if";
pub const FN_NAME: &str = "def";

// Operations
pub const ADD: &str = "add";
pub const MULT: &str = "mul";
pub const SUB: &str = "sub";
pub const DBL: &str = "dbl";
pub const INC: &str = "succ";
pub const DEC: &str = "pred";
pub const EQUALS: &str = "eq";
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

// Boolean
pub const TRUE: &str = "true";
pub const FALSE: &str = "false";

// List
pub const CONS: &str = "cons";
pub const CAR: &str = "car";
pub const CDR: &str = "cdr";
pub const LCONS: &str = "lcons";
pub const LCDR: &str = "lcdr";
pub const LCAR: &str = "lcar";
pub const INDEX: &str = "idx";
pub const EMPTY_LIST: &str = "[]";

// Lambda
pub const LAMBDA: &str = "->";
pub const LAMBDA_TYPE: &str = "lambda";

// Binary operations
pub const COMP_OPR: &str = "$";
pub const COMP_LEFT: &str = "@";
pub const PIPE: &str = ">>";

// Some useful token arrays
pub const SPLIT_TOKENS: [&'static str; 11] = [
    OPEN_EXPR, CLOSE_EXPR, NEWLINE, TAB, VAR_SEP, OPEN_LIST, CLOSE_LIST, SPACE, LAMBDA, COMP_OPR,
    PIPE,
];

pub const DONT_ADD: [&'static str; 5] = [NEWLINE, TAB, VAR_SEP, SPACE, EMPTY];

pub const OPEN_TOKENS: [&'static str; 2] = [OPEN_EXPR, OPEN_LIST];
pub const CLOSE_TOKENS: [&'static str; 2] = [CLOSE_EXPR, CLOSE_LIST];

pub const EXPR_TUP: (&'static str, &'static str) = (OPEN_EXPR, CLOSE_EXPR);
pub const LIST_TUP: (&'static str, &'static str) = (OPEN_LIST, CLOSE_LIST);

// ASTNode types
pub const EXPRESSION: &str = "expression";
pub const LIST: &str = "list";
pub const SYMBOL: &str = "symbol";
pub const NUMBER: &str = "number";
pub const STRING: &str = "string";

// REPL commands
pub const QUIT_STRINGS: [&'static str; 4] = ["quit", "quit()", "exit", "exit()"];

pub const RESERVED_KEYWORDS: [&'static str; 34] = [
    LET_NAME,
    FN_NAME,
    IF_NAME,
    EQUALS,
    PUTS,
    PRINT,
    OR,
    AND,
    IMPORT,
    CHAIN,
    CONS,
    SET,
    GET,
    CAR,
    CDR,
    LAMBDA,
    VAR_SEP,
    OPEN_EXPR,
    CLOSE_EXPR,
    OPEN_LIST,
    CLOSE_LIST,
    TRUE,
    FALSE,
    AND,
    OR,
    LCAR,
    LCDR,
    LCONS,
    GT,
    LT,
    LAMBDA_TYPE,
    COMP_OPR,
    COMP_LEFT,
    PIPE,
];

use std::{collections::HashSet};

lazy_static! {
    pub static ref RESERVED_SET: HashSet<String> = {
        let mut reserved_set = HashSet::new();
        for word in RESERVED_KEYWORDS {
            reserved_set.insert(word.to_string());
        }
        reserved_set
    };
    pub static ref INVALID_SET: HashSet<String> = {
        let mut invalid_set = HashSet::new();
        for word in DONT_ADD {
            invalid_set.insert(word.to_string());
        }

        for word in SPLIT_TOKENS {
            invalid_set.insert(word.to_string());
        }

        invalid_set
    };
}
