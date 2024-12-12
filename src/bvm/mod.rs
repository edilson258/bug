#![allow(unused)]

pub mod engine;
mod frame;
mod stack;

use bug::stdlib::list_natives;
use bug::utils::read_file_bytes;
use bug::Program;
use engine::Engine;
use std::env;

pub fn load_program_binary(file_path: &str) -> Program {
    let contents = match read_file_bytes(&file_path) {
        Ok(xs) => xs,
        Err(err) => {
            eprintln!("{:#?}", err);
            std::process::exit(1);
        }
    };
    match bincode::deserialize(&contents) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("[Error]: Couldn't deserialize your program: {:#?}\n", err);
            std::process::exit(1);
        }
    }
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() <= 1 {
        eprintln!("[Error]: No input file provided");
        std::process::exit(1);
    }
    let program = load_program_binary(&cli_args[1].clone());
    Engine::bootstrap(program, list_natives()).run();
}
