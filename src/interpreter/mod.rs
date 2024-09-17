use super::error::Error;
use super::typing::ast::TProgram;

mod value;
//pub use value::*;

mod eval;
pub use eval::*;

// interpret a program
pub fn eval_program(program: TProgram) -> Result<(Interpreter, i32), Error> {
    let mut interpreter = Interpreter::empty();
    interpreter = interpreter.add_program(program);
    let res = interpreter.eval_main()?.try_into()?;
    Ok((interpreter, res))
}
