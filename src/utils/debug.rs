use super::colored::*;
use crate::args::Args;
use crate::parser::{ast::WTProgram, ParseTree, Parser};
use crate::typing::{ast::TProgram, Typer};

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

pub fn debug_color<T>(args: &Args, printing: bool, name: &str, elm: &T)
where
    T: std::fmt::Display + Colored,
{
    if printing {
        if args.no_color {
            eprintln!("{name} :\n{elm}");
        } else {
            let c_elm = elm.colored();
            let msg = cformat!("<cyan>{name} :</>\n{c_elm}");
            eprintln!("{}", msg);
        };
    }
}

/// debug sexp of node
pub fn debug_sexp(args: &Args, parse_tree: &ParseTree) {
    debug(args, args.debug_sexp, "SEXP", &parse_tree.root_node())
}

/// debug parsed program
pub fn debug_parser(args: &Args, parser: &Parser, prog: &WTProgram) {
    debug_color(args, args.debug_parser, "Parser", parser);
    debug_color(args, args.debug_parser, "Parsed program", prog)
}

/// debug typer program
pub fn debug_typer(args: &Args, typer: &Typer, prog: &TProgram) {
    debug_color(args, args.debug_parser, "Typer", typer);
    debug_color(args, args.debug_typer, "Typed program", prog)
}
