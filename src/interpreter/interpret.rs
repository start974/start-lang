use super::error::UnknownOption;
use super::flag::{DebugFlag, Flag};
use crate::lexer;
use crate::parser::cst::AsIdentifier as _;
use crate::parser::{self, cst};
use crate::typer::ast::Typed as _;
use crate::typer::{self, ast};
use crate::utils::error::{ErrorCode, ErrorReport};
use crate::utils::location::{Located as _, SourceId};
use crate::utils::pretty::Pretty;
use crate::vm;
use ariadne::Span as _;

pub trait Interpreter {
    /// get content
    fn content(&self) -> &str;

    /// get source id
    fn source_id(&self) -> &SourceId;

    /// set error code
    fn set_error_code(&mut self, code: i32);

    /// get error code
    fn get_error_code(&self) -> i32;

    /// continue parsing
    fn continue_parsing(&self) -> bool;

    /// get a lexer with offset
    fn get_offset_source(&self, offset: usize) -> usize;

    /// get mutable reference on typer
    fn mut_typer(&mut self) -> &mut typer::Typer;

    /// get vm
    fn mut_vm(&mut self) -> &mut vm::Env;

    /// set debug parser
    fn set_flag(&mut self, b: bool, flag: Flag);

    /// get debug flags
    fn is_active_debug(&self, debug: DebugFlag) -> bool;

    /// print
    fn print(&mut self, doc: &impl Pretty);

    /// active printing of summarry definition
    fn print_summay(&self, def: &ast::ExpressionDefinition);

    /// pretty debug
    fn debug(&mut self, flag: DebugFlag, doc: &impl Pretty) {
        if self.is_active_debug(flag) {
            self.print(doc);
        }
    }
    /// print error
    fn eprint<E>(&mut self, error: &E)
    where
        E: ErrorReport + ErrorCode;

    /// fail with error
    fn fail<E>(&mut self, error: E)
    where
        E: ErrorReport + ErrorCode,
    {
        self.eprint(&error);
        let code = if self.get_error_code() == 0 {
            error.code()
        } else {
            1
        };
        self.set_error_code(code);
    }

    /// run command expr definition
    fn run_expr_definition(
        &mut self,
        def: cst::ExpressionDefinition,
        doc: Option<ast::Documentation>,
    ) {
        self.mut_typer()
            .definition(&def, doc)
            .map(|def| {
                self.print_summay(&def);
                self.debug(DebugFlag::Typer, &def);
                if self.get_error_code() == 0 {
                    self.mut_vm().add_definition(&def)
                }
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    /// run command type definition
    fn run_type_definition(&mut self, def: cst::TypeDefinition, doc: Option<ast::Documentation>) {
        if let Err(e) = self.mut_typer().type_definition(&def, doc) {
            self.fail(e);
        }
    }

    /// run command eval
    fn run_eval(&mut self, expr: cst::Expression) {
        self.mut_typer()
            .expression(&expr)
            .map(|expr| {
                self.debug(DebugFlag::Typer, &expr);
                if self.get_error_code() == 0 {
                    let value = self.mut_vm().eval(&expr).unwrap();
                    self.print(&value);
                }
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    /// run type of expression
    fn run_typeof(&mut self, expr: cst::Expression) {
        self.mut_typer()
            .expression(&expr)
            .map(|expr| {
                let ty = expr.ty();
                self.print(ty);
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    fn run_help(&mut self, var: cst::help::Variable) {
        match self.mut_typer().help(&var) {
            Ok(help) => self.print(&help),
            Err(err) => self.fail(err),
        }
    }

    /// run command set and unset
    fn run_set(&mut self, b: bool, var: cst::expression::Variable) {
        match var.name() {
            "DebugLexer" => self.set_flag(b, Flag::Debug(DebugFlag::Lexer)),
            "DebugParser" => self.set_flag(b, Flag::Debug(DebugFlag::Parser)),
            "DebugTyper" => self.set_flag(b, Flag::Debug(DebugFlag::Typer)),
            _ => self.fail(UnknownOption::from(var)),
        }
    }

    /// run command
    fn run_command(&mut self, cmd: cst::Command) {
        match cmd.kind {
            cst::CommandKind::ExpressionDefinition { keyword, def } => {
                self.run_expr_definition(*def, keyword.get_doc())
            }
            cst::CommandKind::TypeDefinition { keyword, def } => {
                self.run_type_definition(def, keyword.get_doc())
            }
            cst::CommandKind::Eval { expr, .. } => self.run_eval(expr),
            cst::CommandKind::TypeOf { expr, .. } => self.run_typeof(expr),
            cst::CommandKind::Help { var, .. } => self.run_help(var),
            cst::CommandKind::Set { var, .. } => self.run_set(true, var),
            cst::CommandKind::UnSet { var, .. } => self.run_set(false, var),
        }
    }

    /// lexing content
    fn lex(&mut self, content: &str, offset_source: usize) -> Vec<lexer::MetaToken> {
        let source_id = self.source_id();
        match lexer::lex(source_id.clone(), offset_source, content) {
            Ok(tokens) => tokens,
            Err(errs) => {
                for err in errs {
                    self.fail(err);
                }
                Vec::new()
            }
        }
    }

    /// parse command with lexer tokens
    fn parse(&mut self, tokens: &[lexer::MetaToken]) -> Option<cst::Command> {
        let source_id = self.source_id();
        match parser::parse(source_id.clone(), tokens) {
            Ok(parser::CommandOrEnd::Command(cmd)) => Some(*cmd),
            Ok(parser::CommandOrEnd::End(_)) => None,
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
        if self.get_error_code() != 0 {
            return;
        }

        let mut offset = 0;
        let content_copy = self.content().to_string();

        while self.continue_parsing() {
            let content = &content_copy[offset..];
            if content.is_empty() {
                break;
            }
            let offset_source = self.get_offset_source(offset);
            let tokens = self.lex(content, offset_source);
            match tokens.last() {
                None => break,
                Some(last_token) => {
                    self.debug(DebugFlag::Lexer, &tokens);
                    if let Some(cmd) = self.parse(&tokens) {
                        self.debug(DebugFlag::Parser, &cmd);
                        self.run_command(cmd);
                    }
                    offset += last_token.loc().end() - offset_source;
                }
            }
        }
    }
}
