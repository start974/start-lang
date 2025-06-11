use super::pretty::PrettyWriter;
use crate::utils::error::ErrorWriter;
use std::io::Write;

// ===========================================================================
// Writer trait
// ===========================================================================
pub trait WriterTrait: std::fmt::Write + std::io::Write {}

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

impl std::io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::stdout().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()
    }
}

impl WriterTrait for Stdout {}

pub type StdoutPrettyWriter<T> = PrettyWriter<Stdout, T>;
impl<T> StdoutPrettyWriter<T> {
    pub fn make(theme: T) -> Self {
        PrettyWriter::new(theme, Stdout)
    }
}

pub type StdoutErrorWriter<C, T> = ErrorWriter<C, Stdout, T>;
impl<C, T> StdoutErrorWriter<C, T> {
    pub fn make(theme: T, cache: C) -> Self {
        ErrorWriter::new(theme, Stdout, cache)
    }
}

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

impl std::io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::stderr().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stderr().flush()
    }
}

impl WriterTrait for Stderr {}

pub type StderrPrettyWriter<T> = PrettyWriter<Stderr, T>;
impl<T> StderrPrettyWriter<T> {
    pub fn make(theme: T) -> Self {
        PrettyWriter::new(theme, Stderr)
    }
}

pub type StderrErrorWriter<C, T> = ErrorWriter<C, Stderr, T>;
impl<C, T> StderrErrorWriter<C, T> {
    pub fn make(theme: T, cache: C) -> Self {
        ErrorWriter::new(theme, Stderr, cache)
    }
}

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

impl std::io::Write for NoWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl WriterTrait for NoWrite {}

pub type NoPrettyWriter<T> = PrettyWriter<NoWrite, T>;
impl<T> NoPrettyWriter<T> {
    pub fn make(theme: T) -> Self {
        PrettyWriter::new(theme, NoWrite)
    }
}

pub type NoErrorWriter<C, T> = ErrorWriter<C, NoWrite, T>;
impl<C, T> NoErrorWriter<C, T> {
    pub fn make(theme: T, cache: C) -> Self {
        ErrorWriter::new(theme, NoWrite, cache)
    }
}

// ===========================================================================
// String writer
// ===========================================================================

pub struct StringBuffer(String);

impl Default for StringBuffer {
    fn default() -> Self {
        Self(String::new())
    }
}

impl std::fmt::Write for StringBuffer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}

impl std::io::Write for StringBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.push_str(std::str::from_utf8(buf).unwrap());
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl StringBuffer {
    pub fn get_str(&self) -> &str {
        &self.0
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl WriterTrait for StringBuffer {}

pub type StringPrettyWriter<T> = PrettyWriter<StringBuffer, T>;
impl<T> StringPrettyWriter<T> {
    pub fn make(theme: T) -> Self {
        PrettyWriter::new(theme, StringBuffer::default())
    }
}
pub type StringErrorWriter<C, T> = ErrorWriter<C, StringBuffer, T>;
impl<C, T> StringErrorWriter<C, T> {
    pub fn make(theme: T, cache: C) -> Self {
        ErrorWriter::new(theme, StringBuffer::default(), cache)
    }
}

// ===========================================================================
// Debug Writer
// ===========================================================================

pub struct DebugWriter {
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

impl std::io::Write for DebugWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.is_active {
            Stdout.write(buf)
        } else {
            NoWrite.write(buf)
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        if self.is_active {
            Stdout.flush()
        } else {
            NoWrite.flush()
        }
    }
}

impl WriterTrait for DebugWriter {}

pub type DebugPrettyWriter<T> = PrettyWriter<DebugWriter, T>;
pub type DebugErrorWriter<C, T> = ErrorWriter<C, DebugWriter, T>;
