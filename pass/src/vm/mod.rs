//! Interpreter of `lir`, until the Cranelift backend (v0.2.0-beta.4).

pub mod env;
mod error;
pub mod value;

pub use env::Vm;
pub use value::Value;
