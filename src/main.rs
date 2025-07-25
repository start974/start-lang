use clap::{Parser, Subcommand};
use std::process::exit;

mod file_interpreter;
mod formatter;
mod interpreter;
mod parser;
mod repl;
mod stdlib;
mod typing;
mod utils;
mod vm;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// run repl
    Repl,
    /// interpet a file
    Run { path: String },
    /// format a file
    Format {
        path: String,

        #[arg(long)]
        /// print formatted file
        print: bool,

    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Repl => repl::run(),
        Commands::Run { path } => {
            let path = std::path::PathBuf::from(path);
            let code = file_interpreter::run(&path);
            exit(code)
        }
        Commands::Format { path, print } => {
            let path = std::path::PathBuf::from(path);
            let code = formatter::run(&path, print);
            exit(code)
        }
    }
}
