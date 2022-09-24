/*
* Notation grammar:
*
* <notation>   := "Notation" (<notation-rule> | <notation-group>)
* <group>      := "Group" <name> ":=" <nota-group>
* <rule>       := "Rule" ":" <rule-exp> ":=" <exp>
*
* <nota-group> := ("..." "|")? <name> ("|" <name>)+
*
* <rule-exp>   := (<var> "=" <rule-val>)+
*               | <rule-val>+
*
* <rule-val> := <tag>                            (* tag              *)
*               | <name>                         (* group            *)
*               | <rule-exp> "," <rule-exp>      (* concat           *)
*               | "(" <rule-val>* ")"            (* group            *)
*               | "["<list-val> "]"              (* list             *)
*               | <rule-val> "?"                 (* optional         *)
*               | <rule-val> "*"                 (* 0 or more rep    *)
*               | <rule-val> "+"                 (* minimum 1 rep    *)
*               | <rule-val> "{" <rep> "}"       (* repetition limit *)
*
* <rep>        := [0-9]+ ","                     (* minimum n rep    *)
*               |        "," [0-9]+              (* maximum m rep    *)
*               | [0-9]+ "," [0-9]+              (* n to m rep       *)
*
* <tag>        := "\"", .* "\""
* <name>       := [a-z-A-Z]+, ([-_]*, [A-Z-a-z-0-9]+)*
* <list-val>   := [A-Z-a-z-0-9_-]*
*/
use core::fmt;

pub mod notation_group;
mod utils;
// pub mod notation_env;
// mod rule;
// pub mod notation_rule;

use notation_group::NotationGroup;
// use notation_rule::NotationRule;

// use self::notation_env::NotationEnv;

use super::lexer::utils::{Parse, TokenResult};
use super::lexer::Lexer;

pub enum Notation {
    // Rule(NotationRule),
    Group(Box<NotationGroup>),
}

impl fmt::Display for Notation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Notation::Rule(rule) => write!(f, "{rule}"),
            Notation::Group(group) => write!(f, "{:?}", group),
        }
    }
}

// impl<'l, 'c> ParseContext<'l, 'c, NotationEnv> for Notation {
impl<'l> Parse<'l> for Notation {
    /*
     * parse:
     * <notation>   := "Notation" (<notation-rule> | <notation-group>)
     */
    fn parse(lexer: &'l mut Lexer) -> TokenResult<Box<Self>> {
        //match notation
        lexer.next_token().tag("Notation")?;
        lexer.next_token().whitespaces()?;

        NotationGroup::parse(lexer).map(|g| Box::new(Notation::Group(g)))
    }
}

//--------------------------------------------------
//                      TESTS
//--------------------------------------------------
// #[test]
// fn add_one_rule() {
//     let mut env = NotationEnv::empty();
//     let expr = Position::unkown(
//                 ExprCont::Const(
//                     Position::unkown(
//                         Constant::Nat(0);
//                     ))
//         );
//     let rule = NotationRule{
//         name= "test",
//         rule= Rule::Tag("test"),
//         expr
//         }
//     env.add_rule(rule)
//     assert_eq!(env.get("test"), rule);
// }
