mod ast;
mod checker;
mod lexer;
mod parser;
mod span;
mod token;
mod utils;

use bug::stdlib::list_natives;
use checker::Checker;
use lexer::Lexer;
use parser::Parser;
use std::env;
use utils::read_file;

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
  let ast = match Parser::new(&mut lexer).parse() {
    Ok(ast) => ast,
    Err(err) => {
      eprintln!("[Syntax Error]: {} at {:#?}", err.message, err.location);
      return;
    }
  };
  let mut checker = Checker::new(&ast, list_natives());
  checker.check();
  /*
  if Checker::new(&ast).check().emit_all() > 0 {
    eprintln!("Aborting due to previuos errors.");
    std::process::exit(1);
  }
  let program = generator::CodeGenerator::new(&ast).emit();
  let program_binary = bincode::serialize(&program).unwrap_or_else(|err| {
    eprintln!("[ERROR]: Couldn't serialize your program: {}", err.to_string());
    std::process::exit(1);
  });
  let mut file = std::fs::File::create("out.bin").unwrap();
  file.write(&program_binary).unwrap_or_else(|err| {
    eprintln!("[ERROR] Couldn't save serialized program into file: {}", err);
    std::process::exit(1);
  }); */
}
