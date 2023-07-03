use std::fs::{File,read_to_string};
use std::io::{self, BufRead};
use std::path::Path;
use std::ptr::read;

use crate::evaluator::context_tco::EvalContext;
use crate::message::*;
use crate::lex;
use crate::lexer::*;
use crate::parser::parser::parse_all;
use crate::evaluate_all;
use crate::constants::*;

// for reading and storing functions in file
pub const STL_FILE:&str="stl.txt";

// convert to chars, insert ; everytime brackets goes to 0 (excluding first)
// comments: ignore #
pub fn separate_expressions(file_string:&str)->Result<String> {
    let mut all_chars:Vec<String>=vec![];
    let mut stack:Vec<String>=vec![];
    let mut line=1;
    
    for (idx,char) in file_string.chars().enumerate() {
        let char_string=char.to_string();
        if char_string.eq(STMT_END) && all_chars.last().unwrap().eq(STMT_END) {
            continue;
        }

        all_chars.push(char_string.clone());

        if char_string.eq("\n") {
            line+=1;
            continue;
        }

        if char_string.eq(OPEN_EXPR) {
            stack.push(char_string);
            continue;
        } 
        
        if char_string.eq(CLOSE_EXPR) {
            if stack.is_empty() {   
                let msg=format!("Unbalanced expression at line:{}, index:{}", line, idx);
                return err!(msg);
            }
            stack.pop();

            if stack.is_empty() {
                all_chars.push(STMT_END.to_string());
            }
        }
    }

    let joined=all_chars.join("");

    Ok(joined)
}

pub fn save_file(filename:&str, ctx:EvalContext) {
    for (key,value) in ctx.read().symbol_map.iter() {
        if !BUILTINS.contains(&key.as_str()) {
            println!("str:{}",value.to_string());

        }
    }
}

pub fn import_file(filename:&str, ctx:&mut EvalContext)->Result<()>{
    let file=read_file(filename)?;
    println!("Importing file:{}\n", filename);

    let sep=separate_expressions(&file)?;
    let results=evaluate_all(&sep, ctx)?;

    for string in results {
        println!("{}", string);
    }

    Ok(())
}

pub fn read_file(filename:&str)->Result<String> {
    let read=read_to_string(filename);

    match read {
        Ok(file_string) => Ok(file_string),
        Err(_) => errf!("File '{}' doesn't exist.", filename)
    }
}

// from rust by example
fn read_lines(filename:&str ) -> Result<io::Lines<io::BufReader<File>>> {
    let file_open = File::open(filename);

    match file_open {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(_) => errf!("File '{}' doesn't exist.", filename)
    }
}
