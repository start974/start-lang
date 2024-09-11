use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "filename")]
    pub file_name: Option<String>,

    #[arg(long, help = "Print sexp tree-sitter parser")]
    pub debug_sexp: bool,

    #[arg(long, help = "Print the AST")]
    pub debug_parser: bool,

    #[arg(long, help = "Print the typed program")]
    pub debug_typer: bool,

    #[arg(long, help = "Don't colorize error")]
    pub no_color: bool,
}
