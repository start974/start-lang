//! Low IR: backend-independent code.
//!
//! Built from `tir` by `codegen`, run by the `vm` (Cranelift in
//! v0.2.0-beta.5): no spans, no names to resolve, no type aliases. A file is
//! its definitions only: a global each, with a dense `GlobalId` and its type;
//! its name is only kept as a symbol for debugging. The commands (`Eval`)
//! are expressions of `lir` evaluated by the vm from the globals.

pub mod expression;
pub mod file;
pub mod global;
pub mod ty;

pub use expression::Expression;
pub use expression::constant::{Constant, Nat};
pub use file::File;
pub use global::{Global, GlobalId};
pub use ty::Type;
