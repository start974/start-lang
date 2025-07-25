use clap::{Parser, Subcommand};
use std::process::exit;

mod interpreter;
mod parser;
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
    Format { path: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Repl => interpreter::repl(),
        Commands::Run { path } => {
            let code = interpreter::file(&path);
            exit(code)
        }
        Commands::Format { path: _ } => {
            todo!("Format command not implemented yet");
        }
    }
}
