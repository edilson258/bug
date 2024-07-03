mod core;
mod frame;
mod stack;

use std::env;
use std::process::exit;

use bug::utils::read_file_bytes;
use bug::Program;
use core::Runtime;

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    if cli_args.len() <= 1 {
        eprintln!("[Error]: No input file provided");
        exit(1);
    }

    let contents = match read_file_bytes(&cli_args[1]) {
        Ok(xs) => xs,
        Err(err) => {
            eprintln!("{:#?}", err);
            exit(1);
        }
    };

    let program: Program = match bincode::deserialize(&contents) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("[Error]: Couldn't deserialize program: {:#?}\n", err);
            exit(1);
        }
    };

    Runtime::run(program);
}
