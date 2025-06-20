use super::error;
use crate::interpreter::summary::SummaryDefinition;
use crate::parser::{ast as parser_ast, Parser};
use crate::typing::ast::Typed;
use crate::typing::Typer;
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::location::{SourceCache, SourceId};
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use crate::vm::env::Env as EnvVm;

pub struct Interpreter {
    /// cache of interpreter
    cache: SourceCache,
    /// theme used
    theme: Theme,
    /// type checker
    typer: Typer,
    /// virtual machine environment
    vm_env: EnvVm,
    /// print summary for repl
    repl_mod: bool,
    /// error code occur
    err_code: i32,
    /// debug parser
    debug_parser: bool,
    /// debug typer
    debug_typer: bool,
    /// typer env
    debug_typer_env: bool,
}

impl Interpreter {
    /// Create a new interpreter instance
    pub fn new() -> Self {
        let mut this = Self {
            cache: SourceCache::new(),
            theme: Theme::default_theme(),
            typer: Typer::new(),
            vm_env: EnvVm::new(),
            repl_mod: false,
            err_code: 0,
            debug_parser: false,
            debug_typer: false,
            debug_typer_env: false,
        };
        this.set_std_library();
        this
    }

    fn set_std_library(&mut self) {
        // Load the standard library definitions into the environment
        let content = include_str!("../../assets/stdlib.st");
        let path = std::path::PathBuf::from("../../assets/stdlib.st");
        let source_id = self.cache.set_file(path, content.to_string());
        self.run(source_id);
    }

    /// update repl module
    pub fn set_repl_mod(&mut self, repl_mod: bool) {
        self.repl_mod = repl_mod;
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
        error.eprint(&self.theme, &mut self.cache).unwrap();
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
    pub fn set_file(&mut self, path: &str) -> Result<SourceId, i32> {
        let path = std::path::PathBuf::from(path);
        let content = std::fs::read_to_string(path.clone())
            .map_err(|_| self.eprint(error::ErrorFileRead::new(path.clone())))?;
        let source_id = self.cache.set_file(path, content);
        Ok(source_id)
    }

    /// set repl content
    pub fn add_repl(&mut self, content: &str) -> SourceId {
        self.cache.add_repl(content)
    }

    /// update repl source
    pub fn update_repl(&mut self) -> SourceId {
        self.cache.update_repl()
    }

    /// get last repl source id to access to buffer
    pub fn last_repl_source_id(&self) -> SourceId {
        self.cache.last_repl_source_id()
    }

    /// type and run expression definition
    fn run_expr_definition(&mut self, def: parser_ast::ExpressionDefinition) {
        self.typer
            .expression_definition(&def)
            .map(|def| {
                if self.debug_typer {
                    println!("{}", def.to_string(&self.theme));
                }
                if self.repl_mod {
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
    fn run_type_definition(&mut self, def: parser_ast::TypeDefinition) {
        if let Err(e) = self.typer.type_definition(&def) {
            self.fail(e);
        }
    }

    /// run evaluation
    fn run_eval(&mut self, expr: parser_ast::Expression) {
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
    fn run_typeof(&mut self, expr: parser_ast::Expression) {
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
    fn run_set(&mut self, b: bool, identifier: parser_ast::Identifier) {
        match identifier.name() {
            "DebugParser" => self.debug_parser = b,
            "DebugTyper" => self.debug_typer = b,
            "DebugTyperEnv" => self.debug_typer_env = b,
            _ => self.fail(error::UnknownOption::from(identifier)),
        }
    }

    /// run the interpreter in REPL mode
    pub fn run(&mut self, source_id: SourceId) {
        let source = self.cache.get(&source_id).to_string();
        let mut parser = Parser::new(&source, source_id);

        while let Some(res) = parser.parse() {
            if self.repl_mod && self.err_code != 0 {
                break;
            }
            res.map(|cmd| {
                if self.debug_parser {
                    println!("{}", cmd.to_string(&self.theme));
                }
                match cmd {
                    parser_ast::Command::ExpressionDefinition(def) => self.run_expr_definition(def),
                    parser_ast::Command::TypeDefinition(def) => self.run_type_definition(def),
                    parser_ast::Command::Eval(expr) => self.run_eval(expr),
                    parser_ast::Command::TypeOf(expr) => self.run_typeof(expr),
                    parser_ast::Command::Set(b, id) => self.run_set(b, id),
                }
                if self.debug_typer_env {
                    println!("Typer Env:\n{:#?}", self.typer);
                }
            })
            .unwrap_or_else(|e| self.fail(e));
        }
    }
}
