use clap::{ArgGroup, Parser, Subcommand};
use std::process::exit;

mod file_interpreter;
mod format;
mod interpreter;
mod lexer;
mod lsp;
mod parser;
mod repl;
mod typer;
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

    #[command(group(
        ArgGroup::new("mode")
        .args(["print", "diff", "overwrite"])
        .multiple(false)
    ))]
    /// format a file
    Format {
        path: String,

        #[arg(long)]
        /// print formatted file
        print: bool,

        #[arg(long)]
        /// diff view of formatted file
        diff: bool,

        #[arg(long, default_value_t = true)]
        /// format file inplace
        overwrite: bool,
    },

    /// run lsp
    Lsp,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let code = match cli.command {
        Commands::Repl => {
            repl::run();
            0
        }
        Commands::Run { path } => {
            let path = std::path::PathBuf::from(path);
            file_interpreter::run(&path)
        }
        Commands::Format {
            path,
            print,
            diff,
            overwrite,
        } => {
            let path = std::path::PathBuf::from(path);
            let mode = if print {
                format::Mode::Print
            } else if diff {
                format::Mode::Diff
            } else {
                assert!(
                    overwrite,
                    "If not print or diff, then overwrite must be true"
                );
                format::Mode::Overwrite
            };
            format::run(&path, mode)
        }
        Commands::Lsp => {
            lsp::run().await;
            0
        }
    };
    exit(code)
}
