mod alias;
pub use alias::Alias as TyAlias;
pub use alias::TyAliasEnv;

mod builtin;
pub use builtin::Builtin as TyBuiltin;

mod ty;
pub use ty::Ty;
pub use ty::TyEnv;
pub use ty::Typed;
pub use ty::TypedMut;
