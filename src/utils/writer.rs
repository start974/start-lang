use std::io::Write;

use crate::ast::PrettyWriter;

// ===========================================================================
// Writer trait
// ===========================================================================
pub trait WriterTrait: std::fmt::Write {}

// ===========================================================================
// Stdout writer
// ===========================================================================
#[derive(Default)]
pub struct Stdout;

impl std::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        std::io::stdout()
            .write_all(s.as_bytes())
            .map_err(|_| std::fmt::Error)
    }
}

impl WriterTrait for Stdout {}

pub type StdoutPrettyWriter = PrettyWriter<Stdout>;

// ===========================================================================
// Stderr writer
// ===========================================================================
#[derive(Default)]
pub struct Stderr;

impl std::fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        std::io::stderr()
            .write_all(s.as_bytes())
            .map_err(|_| std::fmt::Error)
    }
}

impl WriterTrait for Stderr {}

pub type StderrPrettyWriter = PrettyWriter<Stderr>;

// ===========================================================================
// NoWrite writer
// ===========================================================================
#[derive(Default)]
pub struct NoWrite;

impl std::fmt::Write for NoWrite {
    fn write_str(&mut self, _: &str) -> std::fmt::Result {
        Ok(())
    }
}

impl WriterTrait for NoWrite {}

pub type NoPrettyWriter = PrettyWriter<NoWrite>;

// ===========================================================================
// String writer
// ===========================================================================

impl WriterTrait for String {}

pub type StringPrettyWriter = PrettyWriter<String>;

// ===========================================================================
// Debug Writer
// ===========================================================================

struct DebugWriter {
    is_active: bool,
}

impl std::fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.is_active {
            Stdout.write_str(s)
        } else {
            NoWrite.write_str(s)
        }
    }
}
