use super::stateResult::StateResult;
use super::traits::State;
use crate::lexer;
use crate::utils::location::SourceId;

pub struct LexerState {
    lexer: lexer::Lexer,
    result: Option<Result<Vec<lexer::MetaToken>, Vec<lexer::Error>>>,
}

impl LexerState {
    pub fn new(source_id: SourceId) -> Self {
        LexerState {
            lexer: lexer::Lexer::new(source_id),
            result: None,
        }
    }
}

impl State for LexerState {
    type Input = String;
    type Output = Vec<lexer::MetaToken>;
    type Error = Vec<lexer::Error>;

    fn add_input(&mut self, input: String) {
        self.lexer.add_content(&input);
    }

    fn run(&mut self) -> StateResult<Self::Output, Self::Error> {
        self.lexer.run().into()
    }
}
