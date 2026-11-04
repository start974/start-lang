//! The outputs of a text that went through every pass (`pass::stdlib::Stdlib::run`),
//! for the tools that run it (`run`, REPL, LSP); the formatter only lexes and
//! parses.

use ir::location::{Span, Spanned};
use ir::tir;
use pass::stdlib::Compiled;
use pass::vm::Value;

/// what the user asked for, in command order
pub enum Output<'a> {
    /// an expression definition: nothing to show
    Defined,
    Value(Value, Span),
    Type(&'a tir::Type, Span),
    Help(&'a tir::Help),
}

/// outputs of the commands of `compiled`, in order
pub fn outputs(compiled: &Compiled) -> impl Iterator<Item = Output<'_>> {
    let mut values = compiled.values.iter().cloned();
    compiled
        .tir
        .commands
        .iter()
        .map(move |cmd| match &cmd.kind {
            tir::CommandKind::ExpressionDefinition(_) => Output::Defined,
            tir::CommandKind::Eval(expr) => Output::Value(
                values.next().expect("the vm computes a value per Eval"),
                expr.span(),
            ),
            tir::CommandKind::TypeOf(expr) => Output::Type(&expr.ty, cmd.span),
            tir::CommandKind::Help(help) => Output::Help(help),
        })
}
