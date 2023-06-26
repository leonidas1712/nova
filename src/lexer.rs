use crate::constants;
#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<String>,
    idx:usize
}

impl Lexer {
    pub fn new(input:String)->Lexer {
        let mut filtered=input;

        for token in constants::SPLIT_TOKENS {
            if constants::DONT_ADD.contains(&token) {
                filtered=filtered.replace(token, constants::SPACE);

            } else {
                let sp=constants::SPACE;
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


pub fn lex() {
    println!("Lexer");
    println!("{}",constants::OPEN_EXPR);
    println!("{}", constants::LET);
}

#[cfg(test)]
pub mod lexer_test {
    use super::Lexer;
    #[test]
    pub fn lexer_test_splits_whitespace() {
        let expr=String::from("     (    if ( eq n 0)\n\t( add a b )\n  )    "); 
        let expected= ["(", "if", "(", "eq", "n", "0", ")", "(", "add", "a", "b", ")", ")"];
        let lex=Lexer::new(expr);

        println!("{:?}", lex);
        assert_eq!(expected.to_vec(), lex.tokens);
    }

    #[test]
    pub fn lexer_test_splits_on_bigger() {
        let expr=String::from("\t(x sum >>  x  $  y z   g ) >> (  z, y -> (add z)  \n)");
        let expected=["(", "x", "sum", ">>", "x", "$", "y", "z", "g", ")", ">>", "(", "z", "y", "->", "(", "add", "z", ")", ")"];
        let lex=Lexer::new(expr);
        assert_eq!(expected.to_vec(), lex.tokens);
    }
}
