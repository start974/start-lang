pub use chumsky::Parser as ParserTrait;
use chumsky::{extra::Full, input::InputRef, prelude::EmptyErr};

pub mod ast;
pub mod error;
