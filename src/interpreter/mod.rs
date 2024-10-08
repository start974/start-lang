use super::error::Errors;
use super::typing::ast::TProgram;

mod value;
pub use value::*;

mod eval;
pub use eval::*;

// interpret a program
pub fn eval_program(program: TProgram) -> Result<(Interpreter, i32), Errors> {
    let mut interpreter = Interpreter::empty();
    interpreter = interpreter.add_program(&program).0;
    let res = interpreter.eval_main()?.try_into()?;
    Ok((interpreter, res))
}
