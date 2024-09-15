mod ast;
mod checker;
mod codegenerator;
mod highlighter;
mod lexer;
mod parser;
mod span;
mod token;
mod utils;

use bug::stdlib::list_natives;
use checker::Checker;
use codegenerator::CodeGenerator;
use lexer::Lexer;
use parser::Parser;
use std::{env, io::Write};
use utils::{get_file_stem, read_file};

fn main() {
  let command_line_args: Vec<String> = env::args().collect();
  if command_line_args.len() <= 1 {
    eprintln!("[Error]: No input file provided");
    std::process::exit(1);
  }
  let file_path = &command_line_args[1];
  let file_content = match read_file(&file_path) {
    Ok(contents) => contents,
    Err(err) => {
      eprintln!("[Error]: Couldn't read file {} {}", command_line_args[1], err.to_string());
      std::process::exit(1);
    }
  };
  let mut lexer = Lexer::new(&file_content);
  let mut ast = match Parser::new(&file_path, &file_content, &mut lexer).parse() {
    Ok(ast) => ast,
    Err(err) => {
      eprint!("{}", err);
      std::process::exit(1);
    }
  };
  let mut checker = Checker::new(&file_path, &file_content, list_natives());
  if let Some(diagnostics) = checker.check(&mut ast) {
    eprint!("{}", diagnostics);
    std::process::exit(1);
  }
  let mut generator = CodeGenerator::setup();
  let program = generator.emit(ast);
  // println!("{:#?}", program);
  let program_binary = bincode::serialize(&program).unwrap_or_else(|err| {
    eprintln!("[ERROR]: Couldn't serialize your program: {}", err.to_string());
    std::process::exit(1);
  });
  let mut out_file = std::fs::File::create(&format!("{}.bin", get_file_stem(file_path))).unwrap();
  out_file.write(&program_binary).unwrap_or_else(|err| {
    eprintln!("[ERROR] Couldn't save serialized program into file: {}", err);
    std::process::exit(1);
  });
}
