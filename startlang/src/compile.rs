//! The passes one after the other on a whole text, for the tools that run it
//! (`run`, REPL, LSP); the formatter only lexes and parses.

use errors::Errors;
use ir::location::{SourceId, Span, Spanned};
use ir::tir;
use pass::lexer::Lexer;
use pass::lowering::Lowering;
use pass::parser::Parser;
use pass::stdlib::Stdlib;
use pass::vm::Value;

/// lex, parse, lower, type, generate and run `text`, whose first byte is at
/// `offset` in `source`; `state` keeps the definitions (stdlib, previous REPL
/// inputs). A pass reports every error it finds, but the next pass only runs
/// if there is none: nothing runs after an error.
pub fn compile(
    state: &mut Stdlib,
    source: SourceId,
    text: &str,
    offset: usize,
) -> Result<Compiled, Errors> {
    let tokens = Lexer::builder()
        .offset(offset)
        .build()
        .run(text)
        .into_result()?;
    let pir = Parser::builder().build().run(tokens).into_result()?;
    let sir = Lowering::builder()
        .source(source)
        .build()
        .run(&pir)
        .into_result()?;
    let tir = state.typer.run(&sir).into_result()?;
    let lir = state.codegen.run(&tir).into_result()?;
    let values = state.vm.run(&lir).into_result()?;
    Ok(Compiled { tir, values })
}

/// a text that went through every pass
pub struct Compiled {
    tir: tir::File,
    /// value of each `Eval`, in order
    values: Vec<Value>,
}

/// what the user asked for, in command order
pub enum Output<'a> {
    Defined(&'a tir::ExpressionDefinition),
    Value(Value, Span),
    Type(&'a tir::Type, Span),
    Help(&'a tir::Help),
}

impl Compiled {
    /// outputs of the commands, in order
    pub fn outputs(&self) -> impl Iterator<Item = Output<'_>> {
        let mut values = self.values.iter().cloned();
        self.tir.commands().iter().map(move |cmd| match &cmd.kind {
            tir::CommandKind::ExpressionDefinition(def) => Output::Defined(def),
            tir::CommandKind::Eval(expr) => Output::Value(
                values.next().expect("the vm computes a value per Eval"),
                expr.span(),
            ),
            tir::CommandKind::TypeOf(expr) => Output::Type(tir::Typed::ty(expr), cmd.span()),
            tir::CommandKind::Help(help) => Output::Help(help),
        })
    }
}
