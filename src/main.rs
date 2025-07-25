use clap::{ArgGroup, Parser, Subcommand};
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

    #[command(group(
        ArgGroup::new("mode")
        .args(["print", "diff", "override_"])
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
        /// override file
        override_: bool,
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
        Commands::Format {
            path,
            print,
            diff,
            override_: _,
        } => {
            let path = std::path::PathBuf::from(path);
            let mode = if print {
                formatter::Mode::Print
            } else if diff {
                formatter::Mode::Diff
            } else {
                // override is true
                formatter::Mode::Override
            };
            let code = formatter::run(&path, mode);
            exit(code)
        }
    }
}
