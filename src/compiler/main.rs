mod ast;
mod lexer;
mod parser;
mod token;

use std::process::exit;

use crate::{lexer::Lexer, parser::Parser};

fn main() {
    let input = "1 + 2".to_string().chars().collect::<Vec<char>>();
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);

    let ast = match p.parse() {
        Ok(ast) => ast,
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
            exit(1)
        }
    };

    println!("{:#?}", ast);
}
