extern crate shellexpand;
use shellexpand::tilde;

use std::env::join_paths;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{read_to_string, write, File, OpenOptions};
use std::io::{self, BufRead};
use std::os;
use std::path::{Path, PathBuf};
use std::ptr::read;

use crate::constants::*;
use crate::evaluate_all;
use crate::evaluator::context_tco::EvalContext;
use crate::lex;
use crate::lexer::*;
use crate::message::*;
use crate::parser::parse_node::LET_NODE_TYPE;
use crate::parser::parser::parse_all;

// for reading and storing functions in file
pub const STL_FILE: &str = "~/rust/nova/stl.txt";
pub const USER_FILE: &str = "~/rust/nova/user.txt";

// convert to chars, insert ; everytime brackets goes to 0 (excluding first)
// comments: ignore #
pub fn separate_expressions(file_string: &str) -> Result<String> {
    let mut all_chars: Vec<String> = vec![];
    let mut stack: Vec<String> = vec![];
    let mut line = 1;

    let mut comment_stack: Vec<bool> = vec![];

    for (idx, char) in file_string.chars().enumerate() {
        let char_string = char.to_string();

        // same character so we cant nest comments e.g # # internal # # - invalid
        // this is why backlash is needed to escape in strings
        if char_string.eq(COMMENT) {
            if comment_stack.is_empty() {
                comment_stack.push(true);
            } else {
                comment_stack.pop();
            }
            continue;
        }

        // comment: don't read
        if !comment_stack.is_empty() {
            continue;
        }

        if char_string.eq(STMT_END) && all_chars.last().unwrap().eq(STMT_END) {
            continue;
        }

        all_chars.push(char_string.clone());

        if char_string.eq("\n") {
            line += 1;
            continue;
        }

        if char_string.eq(OPEN_EXPR) {
            stack.push(char_string);
            continue;
        }

        if char_string.eq(CLOSE_EXPR) {
            if stack.is_empty() {
                let msg = format!("Unbalanced expression at line:{}, index:{}", line, idx);
                return err!(msg);
            }
            stack.pop();

            if stack.is_empty() {
                all_chars.push(STMT_END.to_string());
            }
        }
    }

    let joined = all_chars.join("");

    Ok(joined)
}

// partially written by ChatGPT
pub fn extract_fndef(input: String) -> Result<String> {
    // Find the position of the arrow "=>"
    if let Some(arrow_pos) = input.find(FAT_ARROW) {
        // Extract the ID part
        // Find the position of the opening and closing parentheses
        if let (Some(open_paren_pos), Some(close_paren_pos)) =
            (input.find(OPEN_EXPR), input.find(CLOSE_EXPR))
        {
            let id = input[..open_paren_pos].trim();
            // Extract the arguments part
            let arguments = input[open_paren_pos + 1..close_paren_pos].trim();
            let arguments_vec: Vec<&str> = arguments.split(VAR_SEP).map(str::trim).collect();

            let args = arguments_vec.join(SPACE);

            // Extract the expression part
            let expression = input[arrow_pos + 2..].trim();

            // (def id (x) (add x y))
            let fn_def = format!(
                "{}{} {} {}{}{} {}{}",
                OPEN_EXPR, FN_NAME, id, OPEN_EXPR, args, CLOSE_EXPR, expression, CLOSE_EXPR
            );
            return Ok(fn_def);
        }
    }

    return errf!("Couldn't save function:{}", input);
}

// :import, :del, :list
use std::io::Write;

// full name with ~ expanded
pub fn get_full_path(filename: &str) -> PathBuf {
    let file_path = shellexpand::tilde(filename).to_string();
    let file_path = Path::new(&file_path).to_owned();
    file_path
}

// save context functions to filename
pub fn save_file(filename: &str, ctx: EvalContext) -> std::result::Result<(), io::Error> {
    let full_path = get_full_path(filename);
    let mut file = File::create(full_path)?;

    let mut count = 0;
    for (key, value) in ctx.read().symbol_map.iter() {
        if BUILTINS.contains(&key.as_str()) {
            continue;
        }

        if let Ok(fn_string) = extract_fndef(value.to_string()) {
            file.write(fn_string.as_bytes())?;
            file.write(b";\n")?;
            count += 1;
        }
    }
    println!("");
    println!("Saved {} functions to {}.", count, filename);

    Ok(())
}

pub fn import_file(filename: &str, ctx: &mut EvalContext) -> Result<()> {
    let file = read_file(filename)?;
    println!("Importing file: {}\n", filename);

    let sep = separate_expressions(&file)?;
    let results = evaluate_all(&sep, ctx)?;

    for res in results {
        // println!("Res type in import:{}", res.result_type.to_string());
        if res.result.len() == 0 || res.result_type.to_string().eq(LET_NODE_TYPE) {
            continue;
        }

        println!("{}", res.result);
    }

    Ok(())
}

// get file contents as string
pub fn read_file(filename: &str) -> Result<String> {
    let file_path = get_full_path(filename);

    let read = read_to_string(file_path);

    match read {
        Ok(file_string) => Ok(file_string),
        Err(_) => errf!("File '{}' doesn't exist.", filename),
    }
}

// from rust by example
fn read_lines(filename: &str) -> Result<io::Lines<io::BufReader<File>>> {
    let file_open = File::open(filename);

    match file_open {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(_) => errf!("File '{}' doesn't exist.", filename),
    }
}
