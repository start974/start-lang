use crate::interpreter::Interpreter;
use crate::parser::{
    ast::{WTDefsOrExpr, WTProgram},
    ParseTree, Parser,
};
use crate::typing::{
    ast::{TDefsOrExpr, TProgram},
    Typer,
};
use colored::Colorize;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct DebugFlag(AtomicBool);

impl Default for DebugFlag {
    fn default() -> Self {
        Self(AtomicBool::new(false))
    }
}

impl DebugFlag {
    /// se debug flag
    fn set_debug(&self, value: bool) {
        self.0.store(value, Ordering::Relaxed);
    }

    /// activate debug flag
    pub fn activate(&self) {
        self.set_debug(true);
    }

    /// get debug flag
    pub fn is_active(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

lazy_static! {
    pub static ref DEBUG_SEXP: DebugFlag = DebugFlag::default();
    pub static ref DEBUG_PARSER: DebugFlag = DebugFlag::default();
    pub static ref DEBUG_TYPER: DebugFlag = DebugFlag::default();
    pub static ref DEBUG_INTERPRETER: DebugFlag = DebugFlag::default();
}

/// debug sexp of node
pub fn debug_sexp(parse_tree: &ParseTree) {
    if DEBUG_SEXP.is_active() {
        eprintln!("{}:\n{}", "SEXP".cyan().bold(), parse_tree.root_node());
    }
}

/// debug parsed program
pub fn debug_parser(parser: &Parser) {
    if DEBUG_PARSER.is_active() {
        eprintln!("{}:\n{}", "Parser".cyan().bold(), parser);
    }
}

/// debug parsed program
pub fn debug_parsed_prog(prog: &WTProgram) {
    if DEBUG_PARSER.is_active() {
        eprintln!("{}:\n{}", "Parsed program".cyan().bold(), prog);
    }
}

/// debug parsed definitions or expression
pub fn debug_parsed_defs_or_expr(defs_or_expr: &WTDefsOrExpr) {
    if DEBUG_PARSER.is_active() {
        eprintln!(
            "{}:\n{}",
            "Parsed definitions or expression".cyan().bold(),
            defs_or_expr
        );
    }
}

/// debug typer program
pub fn debug_typer(typer: &Typer) {
    if DEBUG_TYPER.is_active() {
        eprintln!("{}:\n{}", "Typer".cyan().bold(), typer);
    }
}

/// debug typed program
pub fn debug_typed_prog(prog: &TProgram) {
    if DEBUG_TYPER.is_active() {
        eprintln!("{}:\n{}", "Typed program".cyan().bold(), prog);
    }
}

/// debug typed definitions or expression
pub fn debug_typed_defs_or_expr(defs_or_expr: &TDefsOrExpr) {
    if DEBUG_TYPER.is_active() {
        eprintln!(
            "{}:\n{}",
            "Typed definitions or expression".cyan().bold(),
            defs_or_expr
        );
    }
}

/// debug interpreter
pub fn debug_interpreter(interpreter: &Interpreter) {
    if DEBUG_INTERPRETER.is_active() {
        eprintln!("{}:\n{}", "Interpreter".cyan().bold(), interpreter);
    }
}

/// debug main result
pub fn debug_i32_res(res: &i32) {
    if DEBUG_INTERPRETER.is_active() {
        eprintln!("{}: {}", "Return value".cyan().bold(), res);
    }
}
