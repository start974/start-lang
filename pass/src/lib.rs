//! Every compilation pass, one module each; a tool calls them one after the
//! other on a whole file:
//!
//! ```text
//! source ─► lexer ─► parser ─► lowering ─► typing ─► codegen ─► vm
//!             tokens    pir        sir        tir        lir      values
//! ```
//!
//! Each pass is built with its builder (`Lexer::builder()…build()`); its
//! `run` takes a whole file and returns a `Partial`: the next representation,
//! as far as it could be built, and every error it found. A tool goes to the
//! next pass only if there is no error (`Partial::into_result`). The typer, the
//! codegen and the vm keep their state between files (stdlib, REPL inputs);
//! `stdlib::Stdlib::load` gives that state with the stdlib loaded.

pub mod codegen;
pub mod lexer;
pub mod lowering;
pub mod parser;
mod partial;
pub mod stdlib;
pub mod typing;
pub mod vm;

pub use partial::Partial;
