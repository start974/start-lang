use super::error::Error;
use super::typing::ast::TProgram;

mod value;
//pub use value::*;

mod eval;
pub use eval::*;

// interpret a program
pub fn eval_program(program: TProgram) -> Result<i32, Error> {
    let context = Context::empty();
    context.add_program(program).eval_main()
}
