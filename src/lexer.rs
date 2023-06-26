use crate::constants::{DONT_ADD, SPACE, SPLIT_TOKENS};

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<String>,
    idx:usize
}

impl Lexer {
    pub fn new(input:String)->Lexer {
        let mut filtered=input;
        
        for token in SPLIT_TOKENS {
            if DONT_ADD.contains(&token) {
                filtered=filtered.replace(token,SPACE);

            } else {
                let sp=SPACE;
                let replacement=format!("{}{}{}",sp,token,sp);
                filtered = filtered.replace(token,&replacement);
            }
        }

        let tokens:Vec<String>=filtered
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();

        Lexer {
            tokens,
            idx:0
        }
    }
}

impl Iterator for Lexer {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
       return match self.tokens.get(self.idx).map(|x| x.to_owned().to_string()) {
            Some(string) => {
                self.idx+=1;
                Some(string)
            },
            None => None
        }
    }
}


#[cfg(test)]
pub mod lexer_test {
    use super::Lexer;
    #[test]
    pub fn lexer_test_splits_whitespace() {
        let expr=String::from("     (    if ( eq n 0)\n\t( add a b )\n  )    "); 
        let expected= ["(", "if", "(", "eq", "n", "0", ")", "(", "add", "a", "b", ")", ")"];
        let lex=Lexer::new(expr);
        assert_eq!(expected.to_vec(), lex.tokens);
    }

    #[test]
    pub fn lexer_test_splits_on_bigger() {
        let expr=String::from("\t(x sum >>  x  $  y z   g ) >> (  z, y -> (add z)  \n)");
        let expected=["(", "x", "sum", ">>", "x", "$", "y", "z", "g", ")", ">>", "(", "z", "y", "->", "(", "add", "z", ")", ")"];
        let lex=Lexer::new(expr);
        assert_eq!(expected.to_vec(), lex.tokens);
    }

    #[test]
    pub fn lexer_test_iterator() {
        let expr=String::from("  ( let x 2 ) ");
        let mut lex=Lexer::new(expr);

        assert_eq!(lex.next().unwrap(), "(");
        assert_eq!(lex.next().unwrap(), "let");
        assert_eq!(lex.next().unwrap(), "x");
        assert_eq!(lex.next().unwrap(), "2");
        assert_eq!(lex.next().unwrap(), ")");
        assert_eq!(lex.next(), None);
     
    }
}
