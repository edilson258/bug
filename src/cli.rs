use clap::{Arg, Command};

pub fn command_line() -> clap::ArgMatches {
    let matches = Command::new("Bug")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("executes the provided program")
                .arg(Arg::new("file").help("the bug program to run").required(true)),
        )
        .subcommand(
            Command::new("run-bin")
                .about("executes the provided binary program")
                .arg(Arg::new("file").help("the bug binary program to run").required(true)),
        )
        .subcommand(
            Command::new("compile")
                .about("compiles the program to bug bytecode")
                .arg(Arg::new("file").help("the bug program to compile").required(true)),
        )
        .get_matches();

    return matches;
}
