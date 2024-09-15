use crate::args::Args;
use crate::parser::ast::WTProgram;
use crate::parser::ParseTree;
use crate::typing::ast::TProgram;

use color_print::cformat;

pub fn debug<T>(args: &Args, printing: bool, name: &str, elm: &T)
where
    T: std::fmt::Display,
{
    if printing {
        if args.no_color {
            eprintln!("{name} :\n{elm}");
        } else {
            let msg = cformat!("<cyan>{name} :</>\n{elm}");
            eprintln!("{}", msg);
        };
    }
}

/// debug sexp of node
pub fn debug_sexp(args: &Args, parse_tree: &ParseTree) {
    debug(args, args.debug_sexp, "SEXP", &parse_tree.root_node())
}

/// debug parsed program
pub fn debug_parser(args: &Args, prog: &WTProgram) {
    debug(args, args.debug_parser, "Parsed program", prog)
}

/// debug typer program
pub fn debug_typer(args: &Args, prog: &TProgram) {
    debug(args, args.debug_typer, "Typed program", prog)
}
