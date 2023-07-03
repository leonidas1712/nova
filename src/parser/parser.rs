
use std::rc::Rc;

use super::parse_special::*;
use crate::constants::*;
use crate::lexer;
use crate::message::*;
use crate::parser::parse_node::*;

pub(super) enum Special {
    If,
    Let,
    Fn,
}

use Special::*;

impl Special {
    fn get_special(token: String) -> Option<Special> {
        let token = token.as_str();
        match token {
            IF_NAME => Some(If),
            LET_NAME => Some(Let),
            FN_NAME => Some(Fn),
            _ => None,
        }
    }
}

macro_rules! try_spec {
    ($vec:ident, $global:expr) => {
        let try_special = Special::get_special($vec.get(0).unwrap().to_string());
        if try_special.is_some() {
            return parse_special(try_special.unwrap(), $vec, $global);
        }
    };
}

pub const EMPTY_MSG: &'static str = "Can't parse empty expression";

fn get_brackets_error(s: String, open_token: &str) -> String {
    let close_token = if open_token.eq(OPEN_EXPR) {
        CLOSE_EXPR
    } else {
        CLOSE_LIST
    };
    let open_token = open_token.to_string();

    let open_count = s.chars().filter(|x| x.to_string().eq(&open_token)).count();
    let close_count = s
        .chars()
        .filter(|x| x.to_string().eq(&close_token.to_string()))
        .count();

    format!(
        "Excess of {} opening brackets: '{}'.",
        open_count - close_count,
        open_token
    )
}

// Parser
fn parse_list_expression(lex: &mut lexer::Lexer) -> Result<Rc<ASTNode>> {
    let open_token = lex.next().unwrap();
    let mut children: Vec<Rc<ASTNode>> = Vec::new();

    // loop and get child expressions
    let opt_token: Option<&str> = loop {
        match lex.peek().map(|x| x.as_str()) {
            Some(token) if CLOSE_TOKENS.contains(&token) => break Some(token),
            None => break None,
            _ => (),
        }

        let res = parse_expression(lex)?;
        children.push(res);
    };

    if children.len() == 0 {
        let expr = if open_token.eq(OPEN_EXPR) { "()" } else { "[]" };
        if open_token.eq(OPEN_LIST) {
            // handle nil here
        }

        return err!(format!("{}: '{}'", EMPTY_MSG, expr));
    }

    // compare first and last token: should match () or []
    // if we broke out of loop without a closing token => not well formed e.g  (2
    match opt_token {
        Some(last_token) => {
            if last_token.eq(STMT_END) {
                let msg=format!("'{}' can't be used inside an expression.", STMT_END);
                return err!(msg);
            }

            let cmp = (open_token.as_str(), last_token);
            if cmp != EXPR_TUP && cmp != LIST_TUP {
                // return bracket mismatch and index
                let msg = format!(
                    "Mismatched brackets: '{}' for '{}' at index {}.",
                    last_token, open_token, lex.idx
                );
                return err!(msg);
            };
        } // ret index
        None => {
            // let msg=format!("Expression was not well formed: expected bracket at index {}", lex.idx);
            let msg = get_brackets_error(lex.to_string().clone(), &open_token);
            return err!(msg);
        }
    };

    lex.next(); // advance past the last token

    // remove nested expressions: (2) => 2, but not for [2]
    if children.len() == 1 && open_token == OPEN_EXPR {
        let node = children.into_iter().next().unwrap();
        return Ok(node);
    }

    // special
    let first = children.get(0).unwrap();

    // try_spec: bool for global
    // global means whether to take return value to set in outer ctx
    // false: expr just returns normal value

    try_spec!(children, false);

    let node_val = if open_token == OPEN_EXPR {
        ParseExpression(children)
    } else {
        List(children)
    };

    Ok(Rc::new(ASTNode::new(node_val)))
}

pub fn parse_atomic_expression(lex: &mut lexer::Lexer) -> Result<Rc<ASTNode>> {
    let token_opt = lex.next();
    if token_opt.is_none() {
        let msg = format!("Problem parsing expression at index {}.", lex.idx);
        return err!(msg);
    }

    let token = token_opt.unwrap();

    if token.eq(TRUE) {
        return Ok(Rc::new(ASTNode::new(Boolean(true))));
    } else if token.eq(FALSE) {
        return Ok(Rc::new(ASTNode::new(Boolean(false))));
    }

    let try_numeric = token.parse::<i64>();

    let node = match try_numeric {
        Ok(num) => Rc::new(ASTNode::new(Number(num))),

        Err(_) => Rc::new(ASTNode::new(Symbol(token))),
    };

    Ok(node)
}

// recursive
pub fn parse_expression(lex: &mut lexer::Lexer) -> Result<Rc<ASTNode>> {
    let token_peek = lex.peek();
    if let None = token_peek {
        return err!(format!("Empty expression at index {}.", lex.idx));
    }

    let token = token_peek.unwrap().as_str();

    // if first token is ), not well formed
    if CLOSE_TOKENS.contains(&token) {
        return err!(format!("Found '{}' at index: {}", token, lex.idx));
    }

    // list
    if OPEN_TOKENS.contains(&token) {
        return parse_list_expression(lex);
    }
    // Check cases in order, last is atomic expression
    parse_atomic_expression(lex)
}

// for now: return first ASTNode
// once curried functions: do evaluation in order
pub fn parse(lex: &mut lexer::Lexer) -> Result<Rc<ASTNode>> {
    let mut nodes: Vec<Rc<ASTNode>> = Vec::new();

    loop {
        if let None = lex.peek() {
            break;
        }

        let res = parse_expression(lex)?;
        nodes.push(res);

        // use ; to separate top level statements
            // ignore duplicated ';'
        if let Some(token) = lex.peek() {
            if token.eq(STMT_END) {
                while let Some(end) = lex.peek() {
                    if !end.eq(STMT_END) {
                        break
                    }
                    lex.next();
                }
                break;
            }
        }
    }

    // filter out dont add tokens e.g let x 2; - semicolon shouldnt be inside
    let nodes_filtered:Vec<Rc<ASTNode>>=nodes.into_iter()
        .filter(|node| {
            match &node.value {
                Symbol(sym) => {
                    !DONT_ADD.contains(&sym.as_str())
                },
                _ => true
            }
        }).collect();

    if nodes_filtered.len() == 0 {
        let msg = format!("{}:'{}'", EMPTY_MSG, lex.to_string());
        return err!(msg);
    };

    let root: Rc<ASTNode> = if nodes_filtered.len() == 1 {
        nodes_filtered.into_iter().next().unwrap()
    } else {
        // if special: return that, otherwise make expr with nodes
        // global true: so that 'let' without brackets can be used for var assignment
        try_spec!(nodes_filtered, true);
        Rc::new(ASTNode::new(ParseExpression(nodes_filtered)))
    };

    Ok(root)
}

// parse nodes separated by ;
pub fn parse_all(mut lex:lexer::Lexer)->Result<Vec<Rc<ASTNode>>> {
    let mut nodes: Vec<Rc<ASTNode>> = Vec::new();
    loop {
        if let None = lex.peek() {
            break;
        }

        let res = parse(&mut lex)?;
        nodes.push(res);
    }
    Ok(nodes)
}

// Tests
#[cfg(test)]
use lexer::Lexer;
pub mod tests {
    use crate::lex;
    use crate::lexer::Lexer;

    use super::*;
    // parse helpers
    pub fn parse_one(exp: &str) -> String {
        let mut lex = crate::lexer::Lexer::new(exp.to_string()).unwrap();
        // dbg!(&lex);
        let res = parse(&mut lex).unwrap();
        // dbg!(&res);
        res.to_string()
    }

    pub fn get_node_value_strings(v: &ASTNode) -> Option<Vec<String>> {
        v.get_children()
            .map(|children| children.iter().map(|x| x.value.to_string()).collect())
    }

    // checks if parsing the strings == node string
    pub fn test_parse(exprs: Vec<&str>) {
        let it = exprs.iter().map(|s| s.trim().to_string());

        // node strings after parsing
        let res = it.clone().map(|s| parse_one(&s));

        // iter(exp, node string)
        let zip = it.zip(res);
        let zip2: Vec<(String, String)> = zip.clone().collect();

        for tup in zip2.into_iter() {
            let left = tup.0;
            let right = tup.1;
            assert_eq!(left, right);
        }
    }

    // test for main parse function
    #[test]
    pub fn parse_test() {
        assert_eq!(parse_one("(2)"), "2");
        assert_eq!(parse_one("(((((3 4)))))"), "(3 4)");

        // space separated expressions put into one - for now
        assert_eq!(parse_one("add 2 2"), "(add 2 2)");
        assert_eq!(
            parse_one("(fn $ map fn lst) (add 2 3) (sub 3 5)"),
            "((fn $ map fn lst) (add 2 3) (sub 3 5))"
        );

        let exps = vec![
            "(sum (map lst (take 5)) (succ 5) [1,2])",
            // "(if (eq n 0) (recr (pred n)) (recr (succ n)))",
            "(map (sum fn (add 2 3)) >> (rec (add 2 3) lst))",
            "sum",
            "[2]",
            "[1,2,(add 5 6),[3,4,[5,6,(sub 4 5)]]]",
        ];

        test_parse(exps);
    }

    #[test]
    pub fn parse_atomic_test() {
        let lex = &mut Lexer::new("let".to_string()).unwrap();
        let res = parse_atomic_expression(lex).unwrap();
        if let Symbol(v) = &res.value {
            assert_eq!(v, "let");
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn parse_list_expression_test_many() {
        let lex = &mut Lexer::new("(sum (map lst (take 5)) (succ 5) [1,2])".to_string()).unwrap();
        let res = parse_list_expression(lex).unwrap();

        let first_layer = get_node_value_strings(&res);
        assert_eq!(
            first_layer.unwrap(),
            vec!["Symbol", "ParseExpression", "ParseExpression", "List"]
        );

        let snd = res.get_ith_child(1);
        let snd_children = snd.and_then(|node| get_node_value_strings(node));
        assert_eq!(
            snd_children.unwrap(),
            vec!["Symbol", "Symbol", "ParseExpression"]
        );

        let snd = snd.unwrap();
        let take = snd
            .get_ith_child(2)
            .and_then(|node| get_node_value_strings(node));

        assert_eq!(take.unwrap(), vec!["Symbol", "Number"]);
    }

    #[test]
    pub fn parse_list_expression_test_nest() {
        let mut lex = &mut Lexer::new("(2)".to_string()).unwrap();
        let res = parse_list_expression(&mut lex).unwrap();
        if let ParseValue::Number(num) = res.value {
            assert_eq!(num, 2);
        } else {
            assert!(false);
        }

        let lex = &mut Lexer::new("(((((((2)))))))".to_string()).unwrap();
        let res = parse_list_expression(lex).unwrap();
        if let ParseValue::Number(num) = res.value {
            assert_eq!(num, 2);
        } else {
            assert!(false);
        }

        // doesn't flatten a list
        let lex = &mut Lexer::new("[2]".to_string()).unwrap();
        let res = parse_list_expression(lex).unwrap();

        if let ParseValue::List(vc) = &res.value {
            assert_eq!(vc.len(), 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn parse_list_expression_test_err() {
        let lex = &mut Lexer::new("(add".to_string()).unwrap();
        let res = parse_list_expression(lex).unwrap_err();
        // println!("{}", res.format_error());
        assert!(&res.format_error().to_lowercase().contains("excess"));

        let lex = &mut Lexer::new("(1,2]".to_string()).unwrap();
        let res = parse_list_expression(lex).unwrap_err();
        assert!(&res.format_error().contains("Mismatched brackets"));

        let lex = &mut lex!("()");
        let res = parse_list_expression(lex).unwrap_err();
        assert!(res
            .format_error()
            .contains("Can't parse empty expression: '()'"));

        let lex = &mut lex!("(add 2 ())");
        let res = parse_list_expression(lex).unwrap_err();
        assert!(res
            .format_error()
            .contains("Can't parse empty expression: '()'"));
    }

    fn test_equality_expr(s: &str) {
        let mut l = lex!(s);
        let p = parse(&mut l).unwrap();

        let mut l2 = lex!(s);
        let p2 = parse(&mut l2).unwrap();

        println!("{}", p.to_string_with_parent());

        match &p.value {
            ParseExpression(children) | IfNode(children) | LetNode(children, _) => {
                let c1 = children.get(0).unwrap();

                let c1_parent = c1.parent.clone().unwrap();
                let c1_parent = c1_parent.as_ref();
                // let c1_parent=c1_parent.as_ref();

                let c1_parent_cloned = c1_parent.clone();

                // child's parent compares equal to original
                let p_ref = p.as_ref();
                assert_eq!(c1_parent, p_ref);

                // cloned expr should be unequal
                // (fn 1 (fn 2 3)) where (fn 2 3) = (add a b)
                // 1 is in resq when doing (fn 2 3)
                // 1's parent is e here but could be a more nested expr where
                // 1's parent is (add a b) while in queue
                // then fn2 takes too many arguments out
                assert_ne!(c1_parent, &c1_parent_cloned);

                // but same expr is not equal (essentially compare by ref)
                assert_ne!(&p, &p2);
            }
            _ => {}
        }
    }

    #[test]
    pub fn test_equality() {
        test_equality_expr("(add 2 3)");
        // test_equality_expr("(if 1 2 3)");
        // test_equality_expr("(let x 2 y 3)");
    }
}
