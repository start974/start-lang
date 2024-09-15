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
pub fn eval_program(program: TProgram) -> Result<u32, Error> {
    let mut main = None;
    let mut context = Context::empty();
    for def in program.iter() {
        context = context.add_definition(def.clone());
        if def.get_name().name == "main" {
            main = Some((def.get_body(), def.get_location()))
        }
    }
    match main {
        None => Err(Error::error_simple(
            "main function not found",
            ERROR_MAIN_NOT_FOUND,
        )),
        Some((main, opt_loc)) if *main.get_ty() != (*MAIN_TY) => {
            let msg = format!("main function must be typed by '{}' type", *MAIN_TY);
            Err(Error::error_located(
                &msg,
                opt_loc.clone().unwrap(),
                ERROR_MAIN_TYPE,
            ))
        }
        Some((main, _)) => match context.eval_expr(main) {
            Value::N(value) => Ok(value),
        },
    }
}
