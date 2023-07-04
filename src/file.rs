use std::error::Error;
use std::fs::{File,read_to_string,write};
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


pub fn extract_fndef(input:String)->Result<String> {
    // let input = "id(x,y) => (add x y)";

    // Find the position of the arrow "=>"
    if let Some(arrow_pos) = input.find("=>") {
        // Extract the ID part
        // Find the position of the opening and closing parentheses
        if let (Some(open_paren_pos), Some(close_paren_pos)) = (input.find('('), input.find(')')) {
            let id = input[..open_paren_pos].trim();
            // Extract the arguments part
            let arguments = input[open_paren_pos + 1..close_paren_pos].trim();
            let arguments_vec: Vec<&str> = arguments.split(',').map(str::trim).collect();

            let args=arguments_vec.join(" ");

            // Extract the expression part
            let expression = input[arrow_pos + 2..].trim();

            // Print the extracted parts
            println!("ID: {}", id);
            println!("Arguments: {:?}", arguments_vec);
            println!("Expression: {}", expression);

            // (def id (x) (add x y))
            let fn_def=format!("{}{} {} {}{}{} {}{}",OPEN_EXPR,FN_NAME,id,OPEN_EXPR,args,CLOSE_EXPR,expression,CLOSE_EXPR);
            println!("Fndef:{}", fn_def);
            return Ok(fn_def);
        }
    }

    return errf!("Couldn't save function:{}",input);
}

use std::io::Write;
pub fn save_file(filename:&str, ctx:EvalContext)->std::result::Result<(),io::Error> {
    let mut file=File::create(filename)?;

    for (key,value) in ctx.read().symbol_map.iter() {
        if BUILTINS.contains(&key.as_str()) {
            continue;
        }

        if let Ok(fn_string) = extract_fndef(value.to_string()) {
            file.write(fn_string.as_bytes())?;
            file.write(b";\n")?;
        }

    }

    Ok(())
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
