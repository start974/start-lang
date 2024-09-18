pub mod ast;
mod env;
mod infer_type;
use super::error::Errors;
use super::parser::ast::WTProgram;
use crate::stdlib::TYPE_ENV;

pub type TypingEnv = env::TypingEnv;
pub type Typer = infer_type::Typer;

pub fn infer_type(wtprogram: WTProgram) -> Result<(Typer, ast::TProgram), Errors> {
    let typer = Typer::make(TYPE_ENV.clone());
    let res_prog = typer.type_program(&wtprogram);
    res_prog.get_result()
}
