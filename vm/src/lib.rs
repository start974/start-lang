//! Interpreter of `lir`, until the Cranelift backend (v0.2.0-beta.4).

pub mod env;
pub mod value;

pub use env::Env;
pub use value::Value;
