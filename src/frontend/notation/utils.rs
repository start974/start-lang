// use crate::frontend::lexer::{Token, utils::TokenResult};

// // parse
// // <name>       := [a-z-A-Z]+, ([-_]*, [A-Z-a-z-0-9]+)*
// fn parse_name(token: &Token) -> TokenResult<String> {

//     let s, res1 = alpha1(s)?;
//     let s, res2 = many0(|s| ->{
//         let s, res1 = many0(is_a("-_"))(s)?;
//         let s, res2_vec = alphanumeric1(s)?;
//         let res2: String = res2_vec.iter_into().collect();
//         Ok((s, res1 + res2))
//     })(s);
//     Ok((s, res1 + res2))
// }
