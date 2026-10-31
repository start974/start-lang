//! Low IR: backend-independent code.
//!
//! Built from `tir` by `codegen`, run by the `vm` (Cranelift in
//! v0.2.0-beta.4): no spans, no names to resolve, no type aliases. Every
//! definition is a global with a dense `GlobalId`; its name is only kept as
//! a symbol for debugging.

pub mod expression;
pub mod file;
pub mod global;
pub mod ty;

pub use expression::Expression;
pub use expression::constant::{Constant, Nat};
pub use file::{File, Item};
pub use global::{Global, GlobalId};
pub use ty::Type;
