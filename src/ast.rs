mod definition;
mod ident;
mod program;

pub type Ident = ident::Ident;
pub type Env = ident::Env;

/*pub mod typed {*/
/*use super::definition;*/
/*use super::program;*/

/*pub type Definition = definition::Definition;*/
/*pub type Program = program::Program;*/
/*}*/

pub mod untyped {
    use super::definition;
    use super::program;

    pub type Definition = definition::Definition;
    pub type Program = program::Program;
}

/*
pub mod expression;
pub mod ident;
pub mod localised;
pub mod ty;
*/
