// ASTNode: type, value:
    // value: Option<Vec<ASTNode>> or a concrete value
// enum: NodeValue<T>
    // NodeValue::Children(Vec<ASTNode>)
    // NodeValue::Value(T)

use std::fmt::Display;
use std::ops::Deref;

// evaluation: evaluate node => Result<T,E>
    // T: some custom wrapper struct/enum e.g NodeResult
    // then NodeResult has another enum for the different data types
        // e.g NodeResult::Number
        // NodeResult::List
use crate::message::Result;
use crate::{lexer, message::{NovaError, NovaResult}};
use crate::constants::{OPEN_TOKENS,CLOSE_TOKENS,SPACE,VAR_SEP,OPEN_EXPR,CLOSE_EXPR,OPEN_LIST,CLOSE_LIST};

#[derive(Debug, Display)]
pub enum NodeValue {
    Symbol(String),
    Number(usize),
    Expression(Vec<ASTNode>),
    List(Vec<ASTNode>)
}

use NodeValue::*;

// ASTNode
#[derive(Debug)]
pub struct ASTNode {
    value:NodeValue
}

impl ASTNode {
    fn new(value:NodeValue)->ASTNode {
        ASTNode { value }
    }

    fn get_children(&self)->Option<&Vec<ASTNode>> {
        if let Expression(children) | List(children) = &self.value {
            Some(&children)
        } else {
            None
        }
    }

    fn get_ith_child(&self, index:usize)->Option<&ASTNode> {
        self.get_children()
        .and_then(|v| v.get(index))
    }
    
    fn to_string(&self)->String {
        match &self.value {
            Symbol(string) => string.clone(),
            Number(num) => num.to_string(),
            Expression(children) => {
                let v:Vec<String>=children.iter().map(|n| n.to_string()).collect();
                format!("{}{}{}",OPEN_EXPR,v.join(SPACE),CLOSE_EXPR)
            },
            List(children) => {
                let v:Vec<String>=children.iter().map(|n| n.to_string()).collect();
                format!("{}{}{}",OPEN_LIST,v.join(VAR_SEP),CLOSE_LIST)
            }
        }
    }
}

impl Deref for ASTNode {
    type Target = NodeValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


// Parser
pub mod parser {
    use crate::constants::{CLOSE_EXPR, EXPR_TUP, LIST_TUP, OPEN_LIST, OPEN_EXPR};
    use super::*;

    fn parse_list_expression(lex:&mut lexer::Lexer)->Result<ASTNode> {
        let open_token=lex.next().expect("Received empty expression");

        let mut children:Vec<ASTNode>=Vec::new();

        // loop and get child expressions
        let opt_token:Option<&str> = loop {
            match lex.peek().map(|x| x.as_str()) {
                Some(token) if CLOSE_TOKENS.contains(&token) => {
                    break Some(token)
                },
                None => break None,
                _ => ()
            }

            let res=parse_expression(lex)?;
            children.push(res.result);
        };

        // compare first and last token: should match () or []
        // if we broke out of loop without a closing token => not well formed e.g  (2
        match opt_token {
            Some(last_token) => {
                let cmp=(open_token.as_str(),last_token);
                if cmp!=EXPR_TUP && cmp!=LIST_TUP {
                    return Err(NovaError::new("Mismatched brackets."));
                };
            },
            None => return Err(NovaError::new("Expression was not well-formed."))
        };

        lex.next(); // advance past the last token

        // remove nested expressions: (2) => 2
        if children.len()==1 && open_token==OPEN_EXPR {
            let node=children.into_iter().next().unwrap();
            return Ok(NovaResult::new(node));
        }

        let node_val=if open_token==OPEN_EXPR { Expression(children) } else { List(children) };

        return Ok(NovaResult::new(ASTNode::new(node_val)));
    }

    fn parse_atomic_expression(lex:&mut lexer::Lexer)->Result<ASTNode> {
        let token_opt=lex.next();
        if token_opt.is_none() {
           return Err(NovaError::new("Problem parsing expression."))
        }

        let token=token_opt.unwrap();
        let try_numeric=token.parse::<usize>();

        let node=match try_numeric {
            Ok(num) => {
                ASTNode::new(Number(num))
            },

            Err(_) => {
                ASTNode::new(Symbol(token))
            }
        };

        Ok(NovaResult::new(node))
    }
    
    // recursive
    fn parse_expression(lex:&mut lexer::Lexer)->Result<ASTNode>{
        let token_peek=lex.peek();
        if let None = token_peek {
            return Err(NovaError::new("Unrecognised expression."))
        }

        let token=token_peek.unwrap().as_str();

        // if first token is ), not well formed
        if CLOSE_TOKENS.contains(&token) {
            return Err(NovaError::new("Expression is not well formed."))
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
    pub fn parse(mut lex:lexer::Lexer)->Option<ASTNode> {
        let mut nodes:Vec<ASTNode>=Vec::new();
        
        loop {
            if let None=lex.peek() {
                break;
            }

            let res=parse_expression(&mut lex);
            if let Ok(nr) = res {
                nodes.push(nr.result);
            }
        }
        nodes.into_iter().next()
    }

    
    #[cfg(test)]
    use lexer::Lexer;

    fn parse_one(exp:&str)->String {
        let lex=crate::lexer::Lexer::new(exp.to_string()).unwrap().result;
        let res=parse(lex).unwrap();
        res.to_string()
        // parse(lex).unwrap().to_string()
    }

    fn get_node_value_strings(v: &ASTNode)->Option<Vec<String>> {
        v.get_children().map( | children |
            children.iter().map(|x| x.value.to_string()).collect()
        )
    }

    // checks if parsing the strings == node string
    fn test_parse(exprs:Vec<&str>)->bool{
        let it=exprs.iter().map(|s| s.trim().to_string());

        // node strings after parsing
        let res=it.clone()
            .map(|s| parse_one(&s));

        // iter(exp, node string)
        let mut zip=it.zip(res);
        zip.all(|tup| tup.0==tup.1)
    }

    // test for main parse function
    #[test]
    pub fn parse_test() {
        assert_eq!(parse_one("(2)"), "2");
        assert_eq!(parse_one("(((((3 4)))))"), "(3 4)");

        let exps=vec![
            "(sum (map lst (take 5)) (succ 5) [1,2])",
            "(if (eq n 0) (recr (pred n)) (recr (succ n)))",
            "(map (sum fn (add 2 3)) >> (rec (add 2 3) lst))",
            "sum",
            "[2]",
            "[1,2,(add 5 6),[3,4,[5,6,(sub 4 5)]]]"
        ];

        assert!(test_parse(exps));
    }   

    #[test]
    pub fn parse_atomic_test() {

        let lex=&mut Lexer::new("let".to_string()).unwrap().result;
        let res=parser::parse_atomic_expression(lex).unwrap();
        if let Symbol(v)=&res.value {
            assert_eq!(v, "let");
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn parse_list_expression_test_many() {
        let lex=&mut Lexer::new("(sum (map lst (take 5)) (succ 5) [1,2])".to_string()).unwrap().result;
        let res=parser::parse_list_expression(lex).unwrap();

        let first_layer=get_node_value_strings(&res.result);
        assert_eq!(first_layer.unwrap(), vec!["Symbol", "Expression", "Expression", "List"]);

        let snd=res.get_ith_child(1);
        let snd_children=snd.and_then(|node| get_node_value_strings(node));
        assert_eq!(snd_children.unwrap(), vec!["Symbol", "Symbol", "Expression"]);

        let snd=snd.unwrap();
        let take=snd.get_ith_child(2).and_then(|node| get_node_value_strings(node));

        assert_eq!(take.unwrap(), vec!["Symbol", "Number"]); 
    }

    #[test]
    pub fn parse_list_expression_test_nest() {
        let lex=&mut Lexer::new("(2)".to_string()).unwrap().result;
        let res=parser::parse_list_expression(lex).unwrap();
        if let NodeValue::Number(num) = res.value {
            assert_eq!(num,2);
        } else {
            assert!(false);
        }

        let lex=&mut Lexer::new("(((((((2)))))))".to_string()).unwrap().result;
        let res=parser::parse_list_expression(lex).unwrap();
        if let NodeValue::Number(num) = res.value {
            assert_eq!(num,2);
        } else {
            assert!(false);
        }

        // doesn't flatten a list
        let lex=&mut Lexer::new("[2]".to_string()).unwrap().result;
        let res=parser::parse_list_expression(lex).unwrap();

        if let NodeValue::List(vc) = res.result.value {
            assert_eq!(vc.len(),1);
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn parse_list_expression_test_err() {
        let lex=&mut Lexer::new("(add".to_string()).unwrap().result;
        let res=parser::parse_list_expression(lex).unwrap_err();
        assert!(&res.format_error().contains("not well-formed."));

        let lex=&mut Lexer::new("(1,2]".to_string()).unwrap().result;
        let res=parser::parse_list_expression(lex).unwrap_err();
        assert!(&res.format_error().contains("Mismatched brackets"));
    }
}


