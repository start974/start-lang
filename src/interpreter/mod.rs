use super::error::Error;
use super::location::Located;
use super::stdlib::number_n::N_TYPE;
use super::typing::ast::{TProgram, Ty, Typed};
use std::sync::LazyLock;

mod env;
//type EvalEnv = env::EvalEnv;

mod value;
pub type Value = value::Value;

mod eval;
pub type Context = eval::Context;

static MAIN_TY: LazyLock<Ty> = LazyLock::new(|| N_TYPE.clone());

const ERROR_MAIN_NOT_FOUND: i32 = 401;
const ERROR_MAIN_TYPE: i32 = 402;

// interpret a program
pub fn eval_program(program: TProgram) -> Result<i32, Error> {
    let mut context = Context::empty();
    context = context.add_program(program);
    match context.get_main() {
        Some(main) => {
            let expr = context.get(main).unwrap();
            if expr.get_ty() != &*MAIN_TY {
                let loc = main.get_location().clone().unwrap();
                let msg = format!("main function must be typed by '{}' type", *MAIN_TY);
                Err(Error::error_located(&msg, loc, ERROR_MAIN_TYPE))
            } else {
                match context.eval_expr(expr) {
                    Value::N(v) => Ok(v.try_into().unwrap()),
                }
            }
        }
        None => Err(Error::error_simple(
            "main function not found",
            ERROR_MAIN_NOT_FOUND,
        )),
    }
}
