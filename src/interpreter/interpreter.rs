//use crate::interpreter::summary::SummaryDefinition;
//use crate::parser::{ast as parser_ast};
//use crate::typing::ast as typing_ast;
use crate::typing::from_parser::FromParser;
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::location::{SourceCache, SourceId};
//use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use crate::vm::env::Env as EnvVm;

use super::error::ErrorFileRead;

pub struct Interpreter {
    /// cache of interpreter
    cache: SourceCache,
    /// theme used
    theme: Theme,
    /// parser
    //parser: Parser,
    /// type checker
    typer: FromParser,
    /// virtual machine environment
    vm_env: EnvVm,
    /// print summary for repl
    repl_mod: bool,
}

type Result<T> = std::result::Result<T, i32>;

impl Interpreter {
    /// Create a new interpreter instance
    pub fn new() -> Self {
        Self {
            cache: SourceCache::new(),
            theme: Theme::default_theme(),
            typer: FromParser::new(),
            vm_env: EnvVm::new(),
            repl_mod: false,
        }
    }

    /// update repl module
    pub fn set_repl_mod(&mut self, repl_mod: bool) {
        self.repl_mod = repl_mod;
    }

    fn fail<E>(&mut self, error: &E) -> i32
    where
        E: ErrorPrint + ErrorCode,
    {
        error.eprint(&self.theme, &mut self.cache).unwrap();
        error.code()
    }

    /// set file content from a file path
    pub fn set_file(&mut self, path: &str) -> Result<SourceId> {
        let path = std::path::PathBuf::from(path);
        let content = std::fs::read_to_string(path.clone())
            .map_err(|_| self.fail(&ErrorFileRead::new(path.clone())))?;
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

    /*    /// parse source code and return the program*/
    /*fn parse(&mut self, source_id: SourceId) -> Result<Command> {*/
    /*let parser = Parser::new(&self.cache, source_id);*/
    /*parser.parse().map_err(|error| self.fail(&error))*/
    /*}*/

    /*    /// type check the program and return the typed program*/
    /*fn typing(&mut self, program: parser_ast::Program) -> Result<typing_ast::Program> {*/
    /*let program_typed = self.typer*/
    /*.program(&program)*/
    /*.map_err(|error| self.fail(&error))?;*/
    /*//println!("{}", self.typer.to_string(&self.theme));*/
    /*Ok(program_typed)*/
    /*}*/

    /*/// eval program*/
    /*fn eval(&mut self, program: &typing_ast::Program) {*/
    /*for def in program.iter() {*/
    /*self.vm_env.add_definition(def);*/
    /*if self.repl_mod {*/
    /*let summary = SummaryDefinition::from(def);*/
    /*println!("{}", summary.to_string(&self.theme));*/
    /*}*/
    /*}*/
    /*}*/

    /// run the interpreter on the given source id
    pub fn run(&mut self, source_id: SourceId) -> Result<()> {
        // parse loop
        todo!();
        //let program_parse = self.parse(source_id)?;
        //let program_typed = self.typing(program_parse)?;
        //self.eval(&program_typed);
        //Ok(())
    }
}
