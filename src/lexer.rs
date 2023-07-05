extern crate regex;
use regex::{Regex,RegexSet};

use std::collections::HashMap;
use std::hash::Hash;

use crate::constants::*;
use crate::message::*;

#[macro_export]
macro_rules! lex {
    ($inp:expr) => {
        Lexer::new($inp.to_string()).unwrap()
    };
}

pub use lex;

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<String>,
    pub idx: usize,
    original: String,
    stack:Vec<String>
}

// need to split but retain !DONT_ADD

// basic split for commands
pub (crate) fn split_input(input:&str)->Vec<String>{
    let split:Vec<String>=input
    .split(|c:char| INVALID_SET.contains(&c.to_string()))
    .map(|x| x.to_string().trim().to_owned())
    .filter(|x| {
        let x_string=x.to_string();
        x_string.len() > 0 && !INVALID_SET.contains(x_string.as_str())
    }).
    collect();
    split
}

impl Lexer {  
    pub fn new(input: String) -> Result<Lexer> {
        let original = input.clone();
        let mut filtered = input;

        if filtered.len() == 0 {
            return err!("Can't parse an empty string");
        }

        for token in SPLIT_TOKENS {
            if DONT_ADD.contains(&token) {
                filtered = filtered.replace(token, SPACE);
            } else {
                let sp = SPACE;
                let replacement = format!("{}{}{}", sp, token, sp);
                filtered = filtered.replace(token, &replacement);
            }
        }

        let tokens: Vec<String> = filtered
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();

        let lex = Lexer {
            tokens,
            idx: 0,
            original,
            stack:vec![]
        };

        Ok(lex)
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.tokens.clone()
    }

    pub fn to_string(&self) -> &String {
        &self.original
    }

    pub fn peek(&self) -> Option<&String> {
        if self.idx >= self.tokens.len() {
            None
        } else {
            self.tokens.get(self.idx)
        }
    }
}

impl Iterator for Lexer {
    type Item = String;

    // can add comments here
    fn next(&mut self) -> Option<Self::Item> {
        return match self.tokens.get(self.idx).map(|x| x.to_owned().to_string()) {
            Some(string) => {
                self.idx += 1;
                Some(string)
            }
            None => None,
        };
    }
}

#[cfg(test)]
pub mod lexer_test {
    use super::Lexer;
    #[test]
    pub fn lexer_test_splits_whitespace() {
        let expr = String::from("     (    if ( eq n 0)\n\t( add a b )\n  )    ");
        let expected = [
            "(", "if", "(", "eq", "n", "0", ")", "(", "add", "a", "b", ")", ")",
        ];
        let lex = Lexer::new(expr).unwrap();
        assert_eq!(expected.to_vec(), lex.tokens);
    }

    // splits on -> and >>
    #[test]
    pub fn lexer_test_splits_on_bigger() {
        let expr = String::from("\t(x sum >>  x  $  y z   g ) >> (  z, y -> (add z)  \n)");
        let expected = [
            "(", "x", "sum", ">>", "x", "$", "y", "z", "g", ")", ">>", "(", "z", "y", "->", "(",
            "add", "z", ")", ")",
        ];
        let lex = Lexer::new(expr).unwrap();
        assert_eq!(expected.to_vec(), lex.tokens);
    }

    #[test]
    pub fn lexer_test_iterator() {
        let expr = String::from("  ( let x 2 ) ");
        let mut lex = Lexer::new(expr).unwrap();

        assert_eq!(lex.next().unwrap(), "(");
        assert_eq!(lex.next().unwrap(), "let");
        assert_eq!(lex.next().unwrap(), "x");
        assert_eq!(lex.next().unwrap(), "2");
        assert_eq!(lex.next().unwrap(), ")");
        assert_eq!(lex.next(), None);
    }

    #[test]
    pub fn lexer_test_to_vec() {
        let expr = String::from("  ( let x 2 ) ");
        let lex = Lexer::new(expr).unwrap();
        let v = lex.to_vec();
        assert_eq!(v, vec!["(", "let", "x", "2", ")"]);
    }

    #[test]
    pub fn lexer_test_peek() {
        let expr = String::from("  ( let x 2 ) ");
        let mut lex = Lexer::new(expr).unwrap();
        lex.next();
        let fst = lex.peek();
        let snd = lex.peek();
        assert_eq!(fst, snd);

        let expr = String::from("h");
        let mut lex = Lexer::new(expr).unwrap();
        lex.next();
        assert_eq!(lex.peek(), None);
    }
}
