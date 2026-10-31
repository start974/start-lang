use super::Runtime;
use super::flag::{DebugFlag, Flag};
use crate::error;
use driver::lowering::Lowering;
use errors::Error;
use ir::location::{SourceId, Span, Spanned};
use ir::tir::Typed as _;
use ir::{pir, sir, tir};
use pp::pretty::Pretty;

/// value of an `Eval`, printed at the span of its expression
struct ValueAt {
    value: driver::vm::Value,
    span: Span,
}

impl Pretty for ValueAt {
    fn pretty(&self, theme: &pp::Theme) -> pp::Doc<'_> {
        self.value.pretty(theme)
    }
}

impl Spanned for ValueAt {
    fn span(&self) -> Span {
        self.span
    }
}

pub trait Interpreter {
    /// get content
    fn content(&self) -> &str;

    /// get source id
    fn source_id(&self) -> &SourceId;

    /// set error code
    fn set_error_code(&mut self, code: i32);

    /// get error code
    fn error_code(&self) -> i32;

    /// continue parsing
    fn continue_parsing(&self) -> bool;

    /// get a lexer with offset
    fn offset_source(&self, offset: usize) -> usize;

    /// get mutable reference on the lowering of the current source
    fn mut_lowering(&mut self) -> &mut Lowering;

    /// get mutable reference on typer
    fn mut_typer(&mut self) -> &mut driver::typing::Typer;

    /// get runtime: codegen and vm
    fn mut_runtime(&mut self) -> &mut Runtime;

    /// set debug parser
    fn set_flag(&mut self, b: bool, flag: Flag);

    /// get debug flags
    fn is_active_debug(&self, debug: DebugFlag) -> bool;

    /// print
    fn print<Doc>(&mut self, doc: &Doc)
    where
        Doc: Pretty + Spanned;

    /// print the summary of a definition (REPL only)
    fn print_summary(&self, def: &tir::ExpressionDefinition);

    /// pretty debug
    fn debug<Doc>(&mut self, flag: DebugFlag, doc: &Doc)
    where
        Doc: Pretty + Spanned,
    {
        if self.is_active_debug(flag) {
            self.print(doc);
        }
    }
    /// print error
    fn eprint(&mut self, error: &Error);

    /// fail with error
    fn fail(&mut self, error: Error) {
        self.eprint(&error);
        self.set_error_code(if self.error_code() == 0 {
            error.code()
        } else {
            1
        });
    }

    /// run command expr definition
    fn run_expr_definition(
        &mut self,
        def: sir::ExpressionDefinition,
        doc: Option<tir::Documentation>,
    ) {
        self.mut_typer()
            .definition(&def, doc.as_ref())
            .map(|def| {
                self.print_summary(&def);
                self.debug(DebugFlag::Typer, &def);
                if self.error_code() == 0 {
                    self.mut_runtime().define(&def);
                }
            })
            .unwrap_or_else(|errs| {
                for err in errs {
                    self.fail(err)
                }
            })
    }

    /// run command type definition
    fn run_type_definition(&mut self, def: sir::TypeDefinition, doc: Option<tir::Documentation>) {
        if let Err(errs) = self.mut_typer().type_definition(&def, doc.as_ref()) {
            for err in errs {
                self.fail(err);
            }
        }
    }

    /// run command eval
    fn run_eval(&mut self, expr: sir::Expression) {
        self.mut_typer()
            .expression(&expr)
            .map(|expr| {
                self.debug(DebugFlag::Typer, &expr);
                if self.error_code() == 0 {
                    let (_, value) = self.mut_runtime().eval(&expr);
                    self.print(&ValueAt {
                        value,
                        span: expr.span(),
                    });
                }
            })
            .unwrap_or_else(|errs| {
                for err in errs {
                    self.fail(err)
                }
            })
    }

    /// run type of expression
    fn run_typeof(&mut self, expr: sir::Expression) {
        self.mut_typer()
            .expression(&expr)
            .map(|expr| {
                let ty = expr.ty();
                self.print(ty);
            })
            .unwrap_or_else(|errs| {
                for err in errs {
                    self.fail(err)
                }
            })
    }

    fn run_help(&mut self, var: sir::Variable) {
        match self.mut_typer().help(&var) {
            Ok(help) => self.print(&help),
            Err(errs) => {
                for err in errs {
                    self.fail(err)
                }
            }
        }
    }

    /// run command set and unset
    fn run_set(&mut self, b: bool, var: sir::Variable) {
        match var.name() {
            "DebugLexer" => self.set_flag(b, Flag::Debug(DebugFlag::Lexer)),
            "DebugParser" => self.set_flag(b, Flag::Debug(DebugFlag::Parser)),
            "DebugTyper" => self.set_flag(b, Flag::Debug(DebugFlag::Typer)),
            _ => self.fail(error::unknown_option(&var)),
        }
    }

    /// run command: lower it, then type and run it
    fn run_command(&mut self, cmd: pir::Command) {
        let lowering = self.mut_lowering();
        let cmd = lowering.command(&cmd).clone();
        let defs = lowering.defs();
        match cmd.kind {
            sir::CommandKind::ExpressionDefinition(def) => {
                let doc = defs.doc(def.id).cloned();
                self.run_expr_definition(*def, doc)
            }
            sir::CommandKind::TypeDefinition(def) => {
                let doc = defs.doc(def.id).cloned();
                self.run_type_definition(def, doc)
            }
            sir::CommandKind::Eval(expr) => self.run_eval(expr),
            sir::CommandKind::TypeOf(expr) => self.run_typeof(expr),
            sir::CommandKind::Help(var) => self.run_help(var),
            sir::CommandKind::Set(var) => self.run_set(true, var),
            sir::CommandKind::UnSet(var) => self.run_set(false, var),
        }
    }

    /// lexing content
    fn lex(&mut self, content: &str, offset_source: usize) -> Option<ir::token::MetaTokenStream> {
        match driver::lexer::lex(content, offset_source) {
            Ok(tokens) => Some(tokens),
            Err(errs) => {
                for err in errs {
                    self.fail(err);
                }
                None
            }
        }
    }

    /// parse command with lexer tokens
    fn parse(&mut self, tokens: ir::token::MetaTokenStream) -> Option<pir::Command> {
        match driver::parser::parse(tokens) {
            Ok(driver::parser::CommandOrEnd::Command(cmd)) => Some(*cmd),
            Ok(driver::parser::CommandOrEnd::End(_)) => None,
            Err(errs) => {
                for err in errs {
                    self.fail(err);
                }
                None
            }
        }
    }

    /// run the interpreter
    fn run(&mut self) {
        if self.error_code() != 0 {
            return;
        }

        let mut offset = 0;
        let content_copy = self.content().to_string();

        while self.continue_parsing() {
            let content = &content_copy[offset..];
            if content.is_empty() {
                break;
            }
            let offset_source = self.offset_source(offset);
            match self.lex(content, offset_source) {
                None => break,
                Some(tokens) => {
                    let offset_end = tokens.last_offset();
                    self.debug(DebugFlag::Lexer, &tokens);
                    if let Some(cmd) = self.parse(tokens) {
                        self.debug(DebugFlag::Parser, &cmd);
                        self.run_command(cmd);
                    }
                    offset += offset_end - offset_source;
                }
            }
        }
    }
}
