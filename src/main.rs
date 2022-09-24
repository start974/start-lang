mod frontend;
use std::{
    env,
    io::{self, Write},
};

use frontend::{error::Error, lexer::Lexer, parser::Parser};

fn cli() -> Result<(), Error> {
    let mut lexer = Lexer::from_stdin();
    let mut parser = Parser::new(&mut lexer);
    // let mut env = Env::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let nodes = parser.parse().map_err(Error::Parsing)?;
        for node in nodes {
            println!("{node}")
        }
        // let node = env.parse_input();
        // print!("{node}\n")
    }
}

fn eval(path: &String) -> Result<(), Error> {
    let mut lexer = Lexer::from_file(path).map_err(Error::Io)?;
    let _ = Parser::new(&mut lexer).parse().map_err(Error::Parsing)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => cli(),
        2 => eval(&args[1]),
        _ => panic!("Unpected more of one argument!"),
    }
}
