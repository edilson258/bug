mod analysis;
mod ast;
mod codegen;
mod lexer;
mod parser;
mod token;

use std::fs;
use std::io::Write;
use std::process::exit;

use crate::analysis::Analiser;
use crate::codegen::CodeGenerator;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let input = "34 + 35".to_string().chars().collect::<Vec<char>>();
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);

    let mut ast = match p.parse() {
        Ok(ast) => ast,
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
            exit(1)
        }
    };

    let mut analiser = Analiser::make();
    match analiser.analise(&mut ast) {
        Ok(()) => {}
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
            exit(1)
        }
    }

    let mut generator = CodeGenerator::make();
    let program = generator.gen(ast);

    let bin = bincode::serialize(&program).unwrap();

    let mut file = fs::File::create("out.bin").unwrap();
    file.write(&bin).unwrap();
}
