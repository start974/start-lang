//! Every compilation pass, and the `Driver` that runs them one after the
//! other:
//!
//! ```text
//! source ─► lexer ─► parser ─► lowering ─► typing ─► codegen ─► vm
//!             tokens    pir        sir        tir        lir      value
//! ```
//!
//! Each pass is a module; clients usually only need the `Driver` and its
//! `Event`s, and reach a pass directly for a partial job (the formatter only
//! lexes and parses).

pub mod codegen;
pub mod lexer;
pub mod lowering;
pub mod parser;
mod pipeline;
pub mod typing;
pub mod vm;

pub use pipeline::{Driver, Event, Output};
