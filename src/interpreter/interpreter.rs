use crate::parser::{ast as parser_ast, Parser};
use crate::typing::from_parser::FromParser;
use crate::typing::{ast as typing_ast};
use crate::utils::error::Error;
use crate::utils::location::{SourceCache, SourceId};
use crate::utils::theme::Theme;
use crate::utils::writer::StderrErrorWriter;

use super::error::ErrorFileRead;

pub struct Interpreter {
    cache: SourceCache,
    theme: Theme,
}

type Result<T> = std::result::Result<T, i32>;

impl Interpreter {
    /// Create a new interpreter instance
    pub fn new() -> Self {
        let theme = Theme::default_theme();
        let cache = SourceCache::new();
        Self { cache, theme }
    }

    fn fail(&mut self, error: &impl Error) -> i32 {
        StderrErrorWriter::make(&self.theme, &mut self.cache).eprint(error);
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

    /// parse source code and return the program
    fn parse(&mut self, source_id: SourceId) -> Result<parser_ast::Program> {
        let parser = Parser::new(&self.cache, source_id);
        parser.parse().map_err(|error| self.fail(&error))
    }

        /// type check the program and return the typed program
    fn typing(&mut self, program: parser_ast::Program) -> Result<typing_ast::Program> {
        let mut from_parser = FromParser::new();
        from_parser.program(&program)
            .map_err(|error| self.fail(&error))
    }

    /*/// eval program*/
    /*fn eval(&self, program: &typing_ast::Program) -> vm::Value {*/
    /*todo!() }*/

    /// run the interpreter on the given source id
    pub fn run(&mut self, source_id: SourceId) -> Result<()> {
        let program_parse = self.parse(source_id)?;
        let program_typed = self.typing(program_parse)?;
        let _ = program_typed;
        //let value = self.eval(program_typed);
        //value
        Ok(())
    }
}
