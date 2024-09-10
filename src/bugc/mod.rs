mod ast;
mod checker;
mod frontend;
mod generator;
mod span;
mod utils;

use checker::checker::Checker;
use frontend::{lexer::Lexer, parser::Parser};
use std::{env, io::Write};
use utils::read_file;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() <= 1 {
    eprintln!("[Error]: No input file provided");
    std::process::exit(1);
  }
  let file = &args[1];
  let file_content = match read_file(&file) {
    Ok(contents) => contents,
    Err(err) => {
      eprintln!("[Error]: Couldn't read file {} {}", args[1], err.to_string());
      std::process::exit(1);
    }
  };
  let mut lexer = Lexer::new(&file, &file_content);
  let ast = Parser::new(&mut lexer).parse();
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
  });
}
