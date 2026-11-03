//! Every compilation pass, one module each; a tool calls them one after the
//! other on a whole file:
//!
//! ```text
//! source ─► lexer ─► parser ─► lowering ─► typing ─► codegen ─► vm
//!             tokens    pir        sir        tir        lir      values
//! ```
//!
//! Each pass is built with its builder when it takes arguments
//! (`Lexer::builder().offset(n).build()`), else with `new` (`Typer::new()`); its
//! `run` takes the `Checked` file of the pass before and returns, through
//! `Partial::check`, either the next representation, `Checked` (no error), or a
//! `Partial`: the representation as far as it could be built, and every
//! error. The lowerer, the typer, the codegen and the vm keep their state
//! between files (stdlib, REPL inputs); `stdlib::Stdlib` keeps it, and runs
//! every pass on a text.

pub mod codegen;
pub mod lexer;
pub mod lowering;
pub mod parser;
mod partial;
pub mod stdlib;
pub mod typing;
pub mod vm;

pub use partial::{Checked, CheckedResult, Partial};
