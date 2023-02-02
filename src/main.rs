mod frontend;
use std::{
    env,
    io::{self, Write},
    vec,
};

use frontend::parser::grammar::Grammar;
use frontend::{lexer::Lexer, parser::grammar};

// fn cli() {// Result<(), Box<dyn std::error::Error>> {
//     let mut lexer = Lexer::from_stdin();
//     // let mut parser = Parser::new(&mut lexer);
//     // let mut env = Env::new();
//     // loop {
//         print!(">> ");
//         io::stdout().flush().unwrap();
//         // while let Some(token) = lexer.next() {
//         //     println!("{token:?}")
//         // }
//         // let node = env.parse_input();
//         // print!("{node}\n")
//     // }
// }

// fn eval(path: &String) -> Result<(), Box<dyn std::error::Error>> {
//     let mut _lexer = Lexer::from_file(path)?;
//     todo!();
//     // let _ = Parser::new(&mut lexer).parse().map_err(Error::Parsing)?;
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    // match args.len() {
    //     1 => Ok(cli()),
    //     2 => eval(&args[1]),
    //     _ => panic!("Unpected more of one argument!"),
    // }
    let mut grammar = Grammar::new();
    let zero = grammar.new_terminal("0");
    let one = grammar.new_terminal("1");
    let plus = grammar.new_terminal("+");
    let mult = grammar.new_terminal("*");
    let e = grammar.new_nonterminal("E");
    let b = grammar.new_nonterminal("B");
    grammar.add_rule(&b, vec![zero]);
    grammar.add_rule(&b, vec![one]);
    grammar.add_rule(&e, vec![b.clone()]);
    grammar.add_rule(&e, vec![e.clone(), plus, b.clone()]);
    grammar.add_rule(&e, vec![e.clone(), mult, b.clone()]);
    println!("{grammar}");
    Ok(())
}
