pub mod ast;
mod env;
mod infer_type;
use super::error::Error;
use super::parser::ast::WTProgram;

pub type TypingEnv = env::TypingEnv;
pub type Typer = infer_type::Typer;

pub fn infer_type(wtprogram: WTProgram) -> Result<ast::TProgram, Error> {
    let typer = Typer::make();
    let (_, res_prog) = typer.type_program(&wtprogram);
    res_prog
}
