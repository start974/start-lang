mod constant;
mod definition;
mod expression;
mod ident;
mod program;
mod ty;

pub type Ident = ident::Ident;
pub type Env = ident::Env;
pub type Ty = ty::Ty;

/*pub mod typed {*/
/*use super::definition;*/
/*use super::program;*/

/*pub type Definition = definition::Definition;*/
/*pub type Program = program::Program;*/
/*}*/

pub mod untyped {
    use super::definition;
    use super::expression;
    use super::program;
    use super::Ty;

    pub type Definition = definition::Definition<Option<Ty>>;
    pub type Program = program::Program<Option<Ty>>;
    pub type Expression = expression::Expression<Option<Ty>>;
    pub type Constant = expression::Constant;
}

/*
pub mod expression;
pub mod ident;
pub mod localised;
pub mod ty;
*/
