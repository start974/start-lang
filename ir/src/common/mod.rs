//! Pieces shared by every IR. The module is private: each IR re-exports what
//! it uses (`pir::ty::BuiltinType`, `sir::Documentation`, …), so a client never
//! writes `common`.

mod builtin;
mod documentation;

pub use builtin::{BuiltinConstant, BuiltinType};
pub use documentation::Documentation;
