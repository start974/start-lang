use super::error;
use crate::interpreter::summary::SummaryDefinition;
use crate::parser;
use crate::typing::ast::Typed;
use crate::typing::Typer;
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::location::SourceId;
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use crate::vm::env::Env as EnvVm;
use ariadne::Source;
use chumsky::Parser;

pub struct Interpreter {
    /// content cache
    source_id: SourceId,
    /// source
    content: String,
    /// offset in source
    offset_source: usize,
    /// theme used
    theme: Theme,
    /// type checker
    typer: Typer,
    /// virtual machine environment
    vm_env: EnvVm,
    /// error code occur
    err_code: i32,
    /// debug parser
    debug_parser: bool,
    /// debug typer
    debug_typer: bool,
}

impl Interpreter {
    /// Create a new interpreter instance
    pub fn new() -> Self {
        let mut this = Self {
            source_id: SourceId::Unknown,
            content: String::new(),
            offset_source: 0,
            theme: Theme::default_theme(),
            typer: Typer::new(),
            vm_env: EnvVm::new(),
            err_code: 0,
            debug_parser: false,
            debug_typer: false,
        };
        this.set_std_library();
        this
    }

    fn set_std_library(&mut self) {
        // Load the standard library definitions into the environment
        let content = include_str!("../../assets/stdlib.st");
        let path = std::path::PathBuf::from("../../assets/stdlib.st");
        self.source_id = SourceId::File { path };
        self.content = content.to_string();
        self.offset_source = 0;
        self.run();
    }

    /// get error code
    pub fn get_err_code(&self) -> i32 {
        self.err_code
    }

    /// reset error code
    pub fn reset_err_code(&mut self) {
        self.err_code = 0;
    }

    fn eprint<E>(&mut self, error: E) -> i32
    where
        E: ErrorPrint + ErrorCode,
    {
        let source = Source::from(&self.content);
        let mut cache = (self.source_id.clone(), source);
        error.eprint(&self.theme, &mut cache).unwrap();
        error.code()
    }

    fn fail<E>(&mut self, error: E)
    where
        E: ErrorPrint + ErrorCode,
    {
        let code = self.eprint(error);
        self.err_code = if self.err_code == 0 { code } else { 1 };
    }

    /// set file content from a file path
    pub fn set_file(&mut self, path: &str) -> Result<(), i32> {
        let path = std::path::PathBuf::from(path);
        self.content = std::fs::read_to_string(path.clone())
            .map_err(|_| self.eprint(error::ErrorFileRead::new(path.clone())))?;
        self.source_id = SourceId::File { path };
        self.offset_source = 0;
        Ok(())
    }

    fn in_repl_mod(&self) -> bool {
        self.source_id == SourceId::Repl
    }

    /// set repl content
    pub fn add_repl(&mut self, content: &str) {
        self.content = if self.in_repl_mod() {
            format!("{}\n{}", self.content, content).to_string()
        } else {
            self.offset_source = 0;
            self.source_id = SourceId::Repl;
            content.to_string()
        };
    }

    /// type and run expression definition
    fn run_expr_definition(&mut self, def: parser::ast::ExpressionDefinition) {
        self.typer
            .expression_definition(&def)
            .map(|def| {
                if self.debug_typer {
                    println!("{}", def.to_string(&self.theme));
                }
                if self.in_repl_mod() {
                    let summary = SummaryDefinition::from(&def);
                    println!("{}", summary.to_string(&self.theme));
                }
                if self.err_code == 0 {
                    self.vm_env.add_definition(&def)
                }
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    /// run type definition
    fn run_type_definition(&mut self, def: parser::ast::TypeDefinition) {
        if let Err(e) = self.typer.type_definition(&def) {
            self.fail(e);
        }
    }

    /// run evaluation
    fn run_eval(&mut self, expr: parser::ast::Expression) {
        self.typer
            .expression(&expr)
            .map(|expr| {
                if self.debug_typer {
                    println!("{}", expr.to_string(&self.theme));
                }
                if self.err_code == 0 {
                    let value = self.vm_env.eval(&expr);
                    println!("{}", value.to_string(&self.theme));
                }
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    /// run type of expression
    fn run_typeof(&mut self, expr: parser::ast::Expression) {
        self.typer
            .expression(&expr)
            .map(|expr| {
                if self.debug_typer {
                    println!("{}", expr.to_string(&self.theme));
                }
                let ty = expr.ty();
                println!("{}", ty.to_string(&self.theme));
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    /// run set command
    fn run_set(&mut self, b: bool, identifier: parser::ast::Identifier) {
        match identifier.name() {
            "DebugParser" => self.debug_parser = b,
            "DebugTyper" => self.debug_typer = b,
            _ => self.fail(error::UnknownOption::from(identifier)),
        }
    }

    fn run_cmd(&mut self, cmd: parser::ast::Command) {
        match cmd {
            parser::ast::Command::ExpressionDefinition(def) => self.run_expr_definition(def),
            parser::ast::Command::TypeDefinition(def) => self.run_type_definition(def),
            parser::ast::Command::Eval(expr) => self.run_eval(expr),
            parser::ast::Command::TypeOf(expr) => self.run_typeof(expr),
            parser::ast::Command::Set(b, id) => self.run_set(b, id),
        }
    }

    /// run the interpreter in REPL mode
    pub fn run(&mut self) {
        loop {
            let content = self.content[(self.offset_source)..].to_string();
            if content.is_empty() || (self.in_repl_mod() && self.err_code != 0) {
                break;
            }
            let parser = parser::parse::command_offset(self.source_id.clone(), self.offset_source);
            match parser.parse(&content).into_result() {
                Ok((cmd, offset)) => {
                    self.offset_source += offset;
                    if self.debug_parser {
                        println!("{}", cmd.to_string(&self.theme));
                    }
                    self.run_cmd(cmd);
                }
                Err(errs) => {
                    self.fail(
                        errs.iter()
                            .map(|err| {
                                parser::error::Error::new(
                                    err.clone(),
                                    self.source_id.clone(),
                                    self.offset_source,
                                )
                            })
                            .collect::<Vec<_>>(),
                    );
                    self.offset_source = self.content.len();
                    break;
                }
            }
        }
    }
}
