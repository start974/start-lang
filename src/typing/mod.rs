/*pub mod ast;*/
/*use super::error::Errors;*/
/*use super::parser::ast::WTProgram;*/
/*use crate::stdlib::TYPE_ENV;*/

/*mod env;*/
/*pub use env::*;*/

/*mod infer_type;*/
/*pub use infer_type::*;*/

/*pub fn infer_type(wtprogram: WTProgram) -> Result<(Typer, ast::TProgram), Errors> {*/
/*let typer = Typer::make(TYPE_ENV.clone());*/
/*let res_prog = typer.type_program(&wtprogram);*/
/*res_prog.get_result()*/
/*}*/

/*pub fn check_main(typer: &Typer) -> Result<(), Errors> {*/
/*typer.check_main().map_err(Errors::from)*/
/*}*/
