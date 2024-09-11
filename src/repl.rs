//use super::parser::NameEnv;
//use super::parser::Parser;
//use super::stdlib::NAME_ENV;
use rustyline::DefaultEditor;

//struct Env<'a> {
    //parser: Parser<'a>,
//}

/*impl Env {*/
    /*const file_name: String = "<<stdin>>".to_string();*/

    /*pub fn new() -> Self {*/
    /*let parser = Parser::make("<<stdin>>".to_string(), &[], NameEnv::new());*/
        /*Self {*/
            /*NameEnv: NAME_ENV.clone()*/
        /*}*/
    /*}*/

    /*pub parse_string(&self, content: &String) -> ParseTree {*/
        /*ParseTree::of_string("repl".to_string(), content)*/
    /*}*/
/*}*/

pub fn repl() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        match rl.readline(">> ") {
            Ok(mut line) => {
                while !line.ends_with(".") {
                    line.pop();
                    match &rl.readline("-- ") {
                        Ok(line2) => {
                            line += "\n";
                            line += line2
                        }
                        Err(_) => return,
                    }
                }
                line.pop();
                println!("line: {line}")
            }
            Err(_) => return,
        }
    }
}
