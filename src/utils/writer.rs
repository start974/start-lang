use super::pretty::PrettyWriter;
use crate::utils::error::ErrorWriter;

// ===========================================================================
// Stdout writer
// ===========================================================================

pub type StdoutPrettyWriter<T> = PrettyWriter<std::io::Stdout, T>;
impl<T> StdoutPrettyWriter<T> {
    pub fn make(theme: T) -> Self {
        PrettyWriter::new(theme, std::io::stdout())
    }
}

/*pub type StdoutErrorWriter<C, T> = ErrorWriter<C, std::io::Stdout, T>;*/
/*impl<C, T> StdoutErrorWriter<C, T> {*/
    /*pub fn make(theme: T, cache: C) -> Self {*/
        /*ErrorWriter::new(theme, std::io::stdout(), cache)*/
    /*}*/
/*}*/

// ===========================================================================
// Stderr writer
// ===========================================================================

/*pub type StderrPrettyWriter<T> = PrettyWriter<std::io::Stderr, T>;*/
/*impl<T> StderrPrettyWriter<T> {*/
    /*pub fn make(theme: T) -> Self {*/
        /*PrettyWriter::new(theme, std::io::stderr())*/
    /*}*/
/*}*/

pub type StderrErrorWriter<C, T> = ErrorWriter<C, std::io::Stderr, T>;
impl<C, T> StderrErrorWriter<C, T> {
    pub fn make(theme: T, cache: C) -> Self {
        ErrorWriter::new(theme, std::io::stderr(), cache)
    }
}

// ===========================================================================
// Sink writer
// ===========================================================================
/*pub type SinkPrettyWriter<T> = PrettyWriter<std::io::Sink, T>;*/
/*impl<T> SinkPrettyWriter<T> {*/
    /*pub fn make(theme: T) -> Self {*/
        /*PrettyWriter::new(theme, std::io::sink())*/
    /*}*/
/*}*/

/*pub type SinkErrorWriter<C, T> = ErrorWriter<C, std::io::Sink, T>;*/
/*impl<C, T> SinkErrorWriter<C, T> {*/
    /*pub fn make(theme: T, cache: C) -> Self {*/
        /*ErrorWriter::new(theme, std::io::sink(), cache)*/
    /*}*/
/*}*/

// ===========================================================================
// String writer
// ===========================================================================

#[derive(Default)]
pub struct StringBuffer(String);

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

/*    pub fn clear(&mut self) {*/
        /*self.0.clear();*/
    /*}*/
}

pub type StringPrettyWriter<T> = PrettyWriter<StringBuffer, T>;
impl<T> StringPrettyWriter<T> {
    pub fn make(theme: T) -> Self {
        PrettyWriter::new(theme, StringBuffer::default())
    }
}
/*pub type StringErrorWriter<C, T> = ErrorWriter<C, StringBuffer, T>;*/
/*impl<C, T> StringErrorWriter<C, T> {*/
    /*pub fn make(theme: T, cache: C) -> Self {*/
        /*ErrorWriter::new(theme, StringBuffer::default(), cache)*/
    /*}*/
/*}*/

// ===========================================================================
// Debug Writer
// ===========================================================================

/*pub struct DebugWriter {*/
    /*is_active: bool,*/
/*}*/

/*impl std::io::Write for DebugWriter {*/
    /*fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {*/
        /*if self.is_active {*/
            /*std::io::stdout().write(buf)*/
        /*} else {*/
            /*std::io::sink().write(buf)*/
        /*}*/
    /*}*/

    /*fn flush(&mut self) -> std::io::Result<()> {*/
        /*if self.is_active {*/
            /*std::io::stdout().flush()*/
        /*} else {*/
            /*std::io::sink().flush()*/
        /*}*/
    /*}*/
/*}*/

/*pub type DebugPrettyWriter<T> = PrettyWriter<DebugWriter, T>;*/
/*pub type DebugErrorWriter<C, T> = ErrorWriter<C, DebugWriter, T>;*/
