mod frontend;
use std::{
    env,
    io::{self, Write},
};

use frontend::lexer::Lexer;

fn cli() {// Result<(), Box<dyn std::error::Error>> {
    let mut lexer = Lexer::from_stdin();
    // let mut parser = Parser::new(&mut lexer);
    // let mut env = Env::new();
    // loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        // while let Some(token) = lexer.next() {
        //     println!("{token:?}")
        // }
        // let node = env.parse_input();
        // print!("{node}\n")
    // }
}

fn eval(path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut _lexer = Lexer::from_file(path)?;
    todo!();
    // let _ = Parser::new(&mut lexer).parse().map_err(Error::Parsing)?;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => Ok(cli()),
        2 => eval(&args[1]),
        _ => panic!("Unpected more of one argument!"),
    }
}
