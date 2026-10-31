//! Intermediate representations of startlang, one module per stage:
//!
//! - `token`: tokens, with their comments (built by `lexer`);
//! - `pir`: parse IR, concrete syntax tree (built by `parser`);
//! - `sir`: surface IR, untyped abstract syntax tree (built by `lowering`);
//! - `tir`: typed IR, typed abstract syntax tree (built by `typing`);
//! - `lir`: low IR, backend-independent code (built by `codegen`, run by `vm`).
//!
//! See `doc/design/pipeline.md`.

mod common;
pub mod lir;
pub mod location;
pub mod pir;
pub mod sir;
pub mod tir;
pub mod token;
