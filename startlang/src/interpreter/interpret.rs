use super::flag::{DebugFlag, Flag};
use crate::error;
use cst::AsIdentifier as _;
use errors::Error;
use location::{SourceId, Spanned};
use pp::pretty::Pretty;
use syntax::lexer::MetaTokenStream;
use tir::Typed as _;

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
    fn mut_typer(&mut self) -> &mut typing::Typer;

    /// get vm
    fn mut_vm(&mut self) -> &mut vm::Env;

    /// set debug parser
    fn set_flag(&mut self, b: bool, flag: Flag);

    /// get debug flags
    fn is_active_debug(&self, debug: DebugFlag) -> bool;

    /// print
    fn print<Doc>(&mut self, doc: &Doc)
    where
        Doc: Pretty + Spanned;

    /// active printing of summarry definition
    fn print_summay(&self, def: &tir::ExpressionDefinition);

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
        self.set_error_code(if self.get_error_code() == 0 {
            error.code()
        } else {
            1
        });
    }

    /// run command expr definition
    fn run_expr_definition(
        &mut self,
        cst_def: cst::ExpressionDefinition,
        doc: Option<cst::Documentation>,
    ) {
        self.mut_typer()
            .definition(&cst_def, doc)
            .map(|def| {
                self.print_summay(&def);
                self.debug(DebugFlag::Typer, &def);
                if self.get_error_code() == 0 {
                    self.mut_vm().add_definition(&def)
                }
            })
            .unwrap_or_else(|errs| {
                for err in errs {
                    self.fail(err)
                }
            })
    }

    /// run command type definition
    fn run_type_definition(&mut self, def: cst::TypeDefinition, doc: Option<tir::Documentation>) {
        if let Err(errs) = self.mut_typer().type_definition(&def, doc) {
            for err in errs {
                self.fail(err);
            }
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
                    self.print(&(value, expr.span()));
                }
            })
            .unwrap_or_else(|errs| {
                for err in errs {
                    self.fail(err)
                }
            })
    }

    /// run type of expression
    fn run_typeof(&mut self, expr: cst::Expression) {
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

    fn run_help(&mut self, var: cst::help::Variable) {
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
    fn run_set(&mut self, b: bool, var: cst::expression::Variable) {
        match var.name() {
            "DebugLexer" => self.set_flag(b, Flag::Debug(DebugFlag::Lexer)),
            "DebugParser" => self.set_flag(b, Flag::Debug(DebugFlag::Parser)),
            "DebugTyper" => self.set_flag(b, Flag::Debug(DebugFlag::Typer)),
            _ => self.fail(error::unknown_option(&var)),
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
    fn lex(&mut self, content: &str, offset_source: usize) -> Option<MetaTokenStream> {
        use syntax::lexer::lex;
        match lex(content, offset_source) {
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
    fn parse(&mut self, tokens: MetaTokenStream) -> Option<cst::Command> {
        use syntax::parser::{CommandOrEnd, parse};
        match parse(tokens) {
            Ok(CommandOrEnd::Command(cmd)) => Some(*cmd),
            Ok(CommandOrEnd::End(_)) => None,
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
