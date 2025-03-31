use std::{env, fs};

use expr::AstPrinter;
use parser::Parser;
use scanner::Scanner;

mod error;
mod scanner;
mod token;
mod token_type;
mod expr;
mod parser;
mod literal;

pub static HAD_ERROR: bool = false;

fn main() {
    // For now there will be two things in the CLI:
    // 1. Path(-p) -> Give the exact path to the file. (For now we will use this)
    // 2. File(-f) -> Give the file that you want to interpret. I will add this later on
    let cli_options: Vec<String> = env::args().collect();
    if cli_options.get(1).eq(&Some(&"-p".to_string())) {
        // RUN THE FILE
        get_file_contents(cli_options.get(2).unwrap());
    } else {
        panic!("Wrong CLI options used: {:?}", cli_options.get(1));
    }
}

fn get_file_contents(path: &str) {
    let file_content = fs::read_to_string(path);

    match file_content {
        Ok(contents) => {
            println!("FILE CONTENTS: {:?}", contents);
            run(contents);
        },
        Err(err) => {
            print!("No contents found: {err}");
        }
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens_list = scanner.scan_tokens().unwrap();
    println!("TOKENSSS!!!: {:#?}", &tokens_list);
    let mut parser = Parser::new(tokens_list);
    if let Some(expression) = parser.parse() {
        let printer = AstPrinter;
        println!("{}", printer.print(expression));
    }
}