use super::colored::*;
use crate::args::Args;
use crate::interpreter::Interpreter;
use crate::parser::{
    ast::{WTDefsOrExpr, WTProgram},
    ParseTree, Parser,
};
use crate::typing::{
    ast::{TDefsOrExpr, TProgram},
    Typer,
};

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
pub fn debug_parser(args: &Args, parser: &Parser) {
    debug_color(args, args.debug_parser, "Parser", parser)
}

/// debug parsed program
pub fn debug_parsed_prog(args: &Args, prog: &WTProgram) {
    debug_color(args, args.debug_parser, "Parsed program", prog)
}

/// debug parsed definitions or expression
pub fn debug_parsed_defs_or_expr(args: &Args, defs_or_expr: &WTDefsOrExpr) {
    debug_color(
        args,
        args.debug_parser,
        "Parsed definitions or expression",
        defs_or_expr,
    )
}

/// debug typer program
pub fn debug_typer(args: &Args, typer: &Typer) {
    debug_color(args, args.debug_typer, "Typer", typer);
}

/// debug typed program
pub fn debug_typed_prog(args: &Args, prog: &TProgram) {
    debug_color(args, args.debug_typer, "Typed program", prog)
}

/// debug typed definitions or expression
pub fn debug_typed_defs_or_expr(args: &Args, defs_or_expr: &TDefsOrExpr) {
    debug_color(
        args,
        args.debug_typer,
        "Typed definitions or expression",
        defs_or_expr,
    )
}

/// debug interpreter
pub fn debug_interpreter(args: &Args, interpreter: &Interpreter) {
    debug_color(args, args.debug_interpreter, "Interpreter", interpreter);
}

/// debug main result
pub fn debug_i32_res(args: &Args, res: &i32) {
    debug(args, args.debug_interpreter, "Return value", res)
}
