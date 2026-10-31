use errors::Error;
use ir::location::Span;
use ir::{lir, pir, sir, tir};

/// what the user asked for, to print
pub enum Output {
    /// a definition was typed (and run if there was no error before)
    Defined(tir::ExpressionDefinition),
    /// value of an `Eval`, with the span of the expression
    Value(crate::vm::Value, Span),
    /// type of a `TypeOf`
    Type(tir::Type),
    /// answer of a `Help`
    Help(tir::Help),
    /// `Set` (`on`) or `Unset` of an option; the client knows the options
    Option { name: sir::Variable, on: bool },
}

/// what happens during `Driver::run`, in order
pub enum Event<'a> {
    /// lex: tokens of the next command
    Tokens(&'a ir::token::MetaTokenStream),
    /// parse
    Parsed(&'a pir::Command),
    /// lower
    Lowered(&'a sir::Command),
    /// type
    TypedDefinition(&'a tir::ExpressionDefinition),
    TypedExpression(&'a tir::Expression),
    /// codegen
    Generated(&'a lir::Global),
    GeneratedExpression(&'a lir::Expression),
    /// error of any phase
    Error(Error),
    /// result for the user
    Output(Output),
}
