mod constant;
mod definition;
mod expression;
mod ident;
mod program;
mod ty;

/// identifer
pub type Ident = ident::Ident;

/// environment to make identifier
pub type Env = ident::Env;

/// types
pub type Ty = ty::Ty;

/// constant expression
pub type Constant = expression::Constant;

/// kind of expression
pub type ExpressionKind = expression::Kind;

/// expression
pub type Expression<TyT> = expression::Expression<TyT>;

/// definition in program
pub type Definition<TyT> = definition::Definition<TyT>;

/// program
pub type Program<TyT> = program::Program<TyT>;
