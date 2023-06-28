use crate::constants::*;
use crate::lexer;
use crate::message::*;
use crate::parser::node::*;

// Parser
fn parse_list_expression(lex: &mut lexer::Lexer) -> Result<ASTNode> {
    let open_token = lex.next().expect("Received empty expression");

    let mut children: Vec<ASTNode> = Vec::new();

    // tmp node as placeholder
    let mut result = NovaResult::new(ASTNode::empty());

    // loop and get child expressions
    let opt_token: Option<&str> = loop {
        match lex.peek().map(|x| x.as_str()) {
            Some(token) if CLOSE_TOKENS.contains(&token) => break Some(token),
            None => break None,
            _ => (),
        }

        let mut res = parse_expression(lex)?;
        result.add_messages(&mut res);

        let node = res.result;
        children.push(node);
    };

    // compare first and last token: should match () or []
    // if we broke out of loop without a closing token => not well formed e.g  (2
    match opt_token {
        Some(last_token) => {
            let cmp = (open_token.as_str(), last_token);
            if cmp != EXPR_TUP && cmp != LIST_TUP {
                return Err(NovaError::new("Mismatched brackets."));
            };
        }
        None => return Err(NovaError::new("Expression was not well-formed.")),
    };

    lex.next(); // advance past the last token

    // remove nested expressions: (2) => 2, but not for [2]
    if children.len() == 1 && open_token == OPEN_EXPR {
        let node = children.into_iter().next().unwrap();
        return Ok(NovaResult::new(node));
    }

    let node_val = if open_token == OPEN_EXPR {
        Expression(children)
    } else {
        List(children)
    };

    // return Ok(NovaResult::new(ASTNode::new(node_val)));
    result.result = ASTNode::new(node_val);
    Ok(result)
}

fn parse_atomic_expression(lex: &mut lexer::Lexer) -> Result<ASTNode> {
    let token_opt = lex.next();
    if token_opt.is_none() {
        return Err(NovaError::new("Problem parsing expression."));
    }

    let token = token_opt.unwrap();
    let try_numeric = token.parse::<usize>();

    let node = match try_numeric {
        Ok(num) => ASTNode::new(Number(num)),

        Err(_) => ASTNode::new(Symbol(token)),
    };

    Ok(NovaResult::new(node))
}

// recursive
fn parse_expression(lex: &mut lexer::Lexer) -> Result<ASTNode> {
    let token_peek = lex.peek();
    if let None = token_peek {
        return Err(NovaError::new("Unrecognised expression."));
    }

    let token = token_peek.unwrap().as_str();

    // if first token is ), not well formed
    if CLOSE_TOKENS.contains(&token) {
        return Err(NovaError::new("Expression is not well formed."));
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
pub fn parse(mut lex: lexer::Lexer) -> Result<ASTNode> {
    let mut nodes: Vec<ASTNode> = Vec::new();
    let mut result = NovaResult::new(ASTNode::empty());

    loop {
        if let None = lex.peek() {
            break;
        }

        let mut res = parse_expression(&mut lex)?;
        result.add_messages(&mut res);

        let node = res.result;
        nodes.push(node);
    }

    if nodes.len() == 0 {
        return Err(NovaError::new("Parse received empty expression."));
    };

    // nodes.into_iter().next()

    let root: ASTNode = if nodes.len() == 1 {
        nodes.into_iter().next().unwrap()
    } else {
        ASTNode::new(Expression(nodes))
    };

    result.result = root;
    Ok(result)
}

// Tests
#[cfg(test)]
use lexer::Lexer;
pub mod tests {
    use super::*;
    fn parse_one(exp: &str) -> String {
        let lex = crate::lexer::Lexer::new(exp.to_string()).unwrap();
        let res = parse(lex.result).unwrap();
        res.to_string()
        // parse(lex).unwrap().to_string()
    }

    fn get_node_value_strings(v: &ASTNode) -> Option<Vec<String>> {
        v.get_children()
            .map(|children| children.iter().map(|x| x.value.to_string()).collect())
    }

    // checks if parsing the strings == node string
    fn test_parse(exprs: Vec<&str>) -> bool {
        let it = exprs.iter().map(|s| s.trim().to_string());

        // node strings after parsing
        let res = it.clone().map(|s| parse_one(&s));

        // iter(exp, node string)
        let mut zip = it.zip(res);
        zip.all(|tup| tup.0 == tup.1)
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
            "(if (eq n 0) (recr (pred n)) (recr (succ n)))",
            "(map (sum fn (add 2 3)) >> (rec (add 2 3) lst))",
            "sum",
            "[2]",
            "[1,2,(add 5 6),[3,4,[5,6,(sub 4 5)]]]",
        ];

        assert!(test_parse(exps));
    }

    #[test]
    pub fn parse_atomic_test() {
        let lex = &mut Lexer::new("let".to_string()).unwrap().result;
        let res = parse_atomic_expression(lex).unwrap();
        if let Symbol(v) = &res.value {
            assert_eq!(v, "let");
        } else {
            assert!(false);
        }

        // let lex=&mut Lexer::new("(add 2 3)".to_string()).unwrap().result;
        // let res=parser::parse_list_expression(lex).unwrap();
        // dbg!(res);
    }

    #[test]
    pub fn parse_list_expression_test_many() {
        let lex = &mut Lexer::new("(sum (map lst (take 5)) (succ 5) [1,2])".to_string())
            .unwrap()
            .result;
        let res = parse_list_expression(lex).unwrap();

        let first_layer = get_node_value_strings(&res.result);
        assert_eq!(
            first_layer.unwrap(),
            vec!["Symbol", "Expression", "Expression", "List"]
        );

        let snd = res.get_ith_child(1);
        let snd_children = snd.and_then(|node| get_node_value_strings(node));
        assert_eq!(
            snd_children.unwrap(),
            vec!["Symbol", "Symbol", "Expression"]
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
        if let NodeValue::Number(num) = res.value {
            assert_eq!(num, 2);
        } else {
            assert!(false);
        }

        let lex = &mut Lexer::new("(((((((2)))))))".to_string()).unwrap().result;
        let res = parse_list_expression(lex).unwrap();
        if let NodeValue::Number(num) = res.value {
            assert_eq!(num, 2);
        } else {
            assert!(false);
        }

        // doesn't flatten a list
        let lex = &mut Lexer::new("[2]".to_string()).unwrap().result;
        let res = parse_list_expression(lex).unwrap();

        if let NodeValue::List(vc) = res.result.value {
            assert_eq!(vc.len(), 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn parse_list_expression_test_err() {
        let lex = &mut Lexer::new("(add".to_string()).unwrap().result;
        let res = parse_list_expression(lex).unwrap_err();
        assert!(&res.format_error().contains("not well-formed."));

        let lex = &mut Lexer::new("(1,2]".to_string()).unwrap().result;
        let res = parse_list_expression(lex).unwrap_err();
        assert!(&res.format_error().contains("Mismatched brackets"));
    }
}
