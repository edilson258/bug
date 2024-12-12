mod bugc;
mod bvm;
mod utils;

use std::io::Write;

use bug::stdlib::list_natives;
use bugc::compile;
use bugc::utils::get_file_stem;
use bvm::engine::Engine;
use bvm::load_program_binary;

mod cli;

fn main() {
    let matches = cli::command_line();

    match matches.subcommand() {
        Some(("run", matches)) => {
            let file_path = matches.get_one::<String>("file").unwrap();
            let program = compile(file_path);
            Engine::bootstrap(program, list_natives()).run();
        }
        Some(("run-bin", matches)) => {
            let file_path = matches.get_one::<String>("file").unwrap();
            let program = load_program_binary(file_path);
            Engine::bootstrap(program, list_natives()).run();
        }
        Some(("compile", matches)) => {
            let file_path = matches.get_one::<String>("file").unwrap();
            let program = compile(file_path);
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
        _ => panic!("No valid command was provided."),
    }
}
