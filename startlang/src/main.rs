use clap::{ArgGroup, Parser, Subcommand};
use std::path::PathBuf;
use std::process::exit;

mod compile;
mod error;
mod format;
mod lsp;
mod printer;
mod repl;
mod run;

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
    /// interpret a file
    Run { path: PathBuf },

    #[command(group(
        ArgGroup::new("mode")
        .args(["print", "diff", "overwrite"])
        .multiple(false)
    ))]
    /// format a file
    Format {
        path: PathBuf,

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
        Commands::Run { path } => run::run(&path),
        Commands::Format {
            path,
            print,
            diff,
            overwrite,
        } => {
            let mode = if print {
                format::Mode::Print
            } else if diff {
                format::Mode::Diff
            } else {
                debug_assert!(
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
