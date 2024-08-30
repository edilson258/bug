mod ast;
mod frontend;
mod utils;

use std::env;

use frontend::{lexer::Lexer, parser::Parser};
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
  let mut parser = Parser::new(&mut lexer);
  println!("{:#?}", parser.parse());
}
