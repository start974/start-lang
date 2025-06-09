use crate::{interpreter, parser, typing};

pub fn interpret_file(file_name: &str) {
    let res = Ok(file_name)
        // parse tree
        .and_then(parser::parse_file)
        //.inspect(debug_sexp)
        // parser program
        .and_then(parser::make_program)
        //.inspect(|(parser, prog)| {
            //debug_parser(parser);
            //debug_parsed_prog(prog)
        //})
        // type program
        .map(|(_, prog)| prog)
        .and_then(typing::infer_type)
        //.inspect(|(typer, prog)| {
            //debug_typer(typer);
            //debug_typed_prog(prog)
        //})
        // check main exists and well typed
        .and_then(|(typer, prog)| typing::check_main(&typer).map(|()| prog))
        // interpret program
        .and_then(interpreter::eval_program)
        //.inspect(|(interpreter, ret)| {
            //debug_interpreter(interpreter);
            //debug_i32_res(ret)
        //})
        // map result
        .map(|(_, ret)| ret);
    match res {
        Ok(ret_code) => std::process::exit(ret_code),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(err.get_code())
        }
    }
}
