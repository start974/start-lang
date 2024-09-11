//use super::parser::{Parser, ParseTree};
//use super::parser::ast::{WTDefinition, WTExpression};
//use super::stdlib::NAME_ENV;
use rustyline::DefaultEditor;

/*struct Env<'a> {*/
    /*parser: Parser<'a>,*/
/*}*/

/*enum WTExprOrDef {*/
    /*Expr(WTExpression),*/
    /*Def(WTDefinition)*/
/*}*/

/*const FILE_NAME: &str = "<<stdin>>";*/
//const ERROR_EXPR_OR_DEF: i32 = 201;

/*impl<'a> Env<'a> {*/
    /*fn new() -> Self {*/
        /*let parser = Parser::make(FILE_NAME, &[], NAME_ENV.clone());*/
        /*Self { parser }*/
    /*}*/

    /*fn ok<T>(self, val: T) -> ResultEnv<T> {*/
        /*(self, Ok(val))*/
    /*}*/

    /*fn from_parser_result<T>(mut self, pres: ParserResult<'a, T>) -> ResultEnv<T> {*/
        /*self.parser = pres.parser();*/
        /*let res = pres.result();*/
        /*(self, res)*/
    /*}*/

    /*fn parse_string(mut self, content: &String) -> Result<ExprOrDef {*/
        /*let node = ParseTree::of_string(FILE_NAME, content).root_node();*/
        /*let (parser, res) = match node.kind() {*/
            /*"expression" => {*/
                /*self.parser.parse_expression(&node);*/
            /*}*/
            /*"definition" => {*/
                /*self.parser.parse_definition(&node);*/
            /*}*/
            /*_ => {*/
                /*let location = self.parser.location(&node);*/
                /*let err = Error::error_located(*/
                    /*"Expected expression or definition",*/
                    /*location, 201);*/

                /*(self.parser, Err(err))*/
            /*}*/
        /*}*/
        /*self.parser = parser;*/
        /*(self, res)*/
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
