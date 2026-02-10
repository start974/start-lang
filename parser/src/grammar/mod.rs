mod class;

pub use class::Class;

#[derive(Debug, Clone)]
pub enum Syntax {
    // ----------------------------
    // Atomes de base
    // ----------------------------
    //Literal(String), // "abc"
    Class(Class), // [a-z], [^0-9]
                  /*    RuleRef(String), // référence à une règle existante*/
                  /*WsOpt,           // '-' : whitespace optionnel*/
                  /*WsReq,           // '_' : whitespace obligatoire*/

                  /*// ----------------------------*/
                  /*// Composition PEG*/
                  /*// ----------------------------*/
                  /*Seq(Vec<Syntax>),    // a b c*/
                  /*Choice(Vec<Syntax>), // a / b / c*/
                  /*Group(Box<Syntax>),  // ( ... )*/

                  /*Optional(Box<Syntax>),                          // a?*/
                  /*Repeat0(Box<Syntax>),                           // a**/
                  /*Repeat1(Box<Syntax>),                           // a+*/
                  /*RepeatRange(Box<Syntax>, usize, Option<usize>), // a{n,m}*/

                  /*Not(Box<Syntax>), // !a*/

                  /*// ----------------------------*/
                  /*// Pratt / opérateurs*/
                  /*// ----------------------------*/
                  /*Prefix {*/
                      /*op: String,       // opérateur préfixe, ex: "type", "!"*/
                      /*rhs: Box<Syntax>, // opérande*/
                      /*bp: u8,           // binding power*/
                  /*},*/
                  /*Postfix {*/
                      /*lhs: Box<Syntax>, // opérande*/
                      /*op: String,       // opérateur postfix, ex: repeat, guard*/
                      /*bp: u8,           // binding power*/
                  /*},*/
                  /*Infix {*/
                      /*lhs: Box<Syntax>,  // opérande gauche*/
                      /*op: String,        // opérateur infix, ex: +, *, etc.*/
                      /*rhs: Box<Syntax>,  // opérande droite*/
                      /*bp: u8,            // binding power*/
                      /*right_assoc: bool, // true si associatif à droite*/
                  /*},*/

                  /*// ----------------------------*/
                  /*// Template / AST lié*/
                  /*// ----------------------------*/
                  /*TemplateVar(String), // variable capturée pour AST*/
}
