use std::rc::Rc;

use crate::location2::{Location, Report, ReportBuilder, SourceCache, SourceId};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use crate::utils::writer::{StringBuffer, StringPrettyWriter};

// ===========================================================================
// Error Writer
// ===========================================================================

pub struct ErrorWriter<W, T> {
    cache: SourceCache,
    writer: W,
    theme: T,
}

impl<W, T> ErrorWriter<W, T> {
    pub fn new(theme: T, writer: W, cache: SourceCache) -> Self {
        Self {
            cache,
            theme,
            writer,
        }
    }
}
impl<W, T> ErrorWriter<W, T>
where
    W: std::io::Write,
    T: AsRef<Theme>,
{
    /// print error with writer
    pub fn eprint(&mut self, e: &impl ErrorWrite) {
        e.write(self.theme.as_ref(), &mut self.cache, &mut self.writer);
    }
}

// ===========================================================================
// Message
// ===========================================================================
enum MessageKind {
    Text(String),
    Important(String),
}
pub struct Message(Vec<MessageKind>);
impl Message {
    /// empty message
    pub fn nil() -> Self {
        Self(Vec::new())
    }

    /// text message
    pub fn text(mut self, text: &str) -> Self {
        self.0.push(MessageKind::Text(text.to_string()));
        self
    }

    /// important message part
    pub fn important(mut self, text: &str) -> Self {
        self.0.push(MessageKind::Important(text.to_string()));
        self
    }

    /// import text quoted
    pub fn quoted(self, text: &str) -> Self {
        self.important(&format!("\"{}\"", text))
    }
}

impl Pretty for Message {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.0.iter().map(|part| match part {
                MessageKind::Text(text) => theme.error_message(text),
                MessageKind::Important(text) => theme.error_important(text),
            }),
            Doc::space(),
        )
    }
}

// ===========================================================================
// Error trait
// ===========================================================================

trait ErrorCode {
    /// error code
    fn code(&self) -> i32;
}

trait ErrorReport: ErrorCode {
    /// location of error
    fn location(&self) -> Location;

    /// finalize report
    fn finalize(&self, theme: &Theme, report: ReportBuilder) -> Report;

    /// message of error
    fn message(&self, theme: &Theme) -> Message;

    /// string of message
    fn message_string(&self, theme: &Theme) -> String {
        let mut writer = StringPrettyWriter::make(&theme);
        writer.print(&self.message(theme));
        writer.writer_mut().get_str().to_string()
    }

    /// report of error
    fn report(&self, theme: &Theme) -> Report {
        self.finalize(
            theme,
            Report::build(ariadne::ReportKind::Error, self.location())
                .with_code(self.code())
                .with_message(self.message_string(theme)),
        )
    }
}

trait ErrorWrite {
    /// write error
    fn write(&self, theme: &Theme, cache: &mut SourceCache, writer: &mut dyn std::io::Write);
}

// ===========================================================================
// Error Report
// ===========================================================================
struct ErrorReportBuilder<T>(T);

impl<T> ErrorReportBuilder<T> {
    /// create new error report
    pub fn new(error_report: T) -> Self {
        Self(error_report)
    }
}

impl<T> ErrorWrite for ErrorReportBuilder<T>
where
    T: ErrorReport,
{
    fn write(&self, theme: &Theme, cache: &mut SourceCache, writer: &mut dyn std::io::Write) {
        self.0.report(theme).write(cache, writer).unwrap();
    }
}

// ===========================================================================
// Error Pair
// ===========================================================================
impl<T> ErrorCode for (T, T)
where
    T: ErrorCode,
{
    fn code(&self) -> i32 {
        let code1 = self.0.code();
        let code2 = self.1.code();
        if code1 == code2 {
            code1
        } else {
            1
        }
    }
}

impl<T> ErrorWrite for (T, T)
where
    T: ErrorWrite,
{
    fn write(&self, theme: &Theme, cache: &mut SourceCache, writer: &mut dyn std::io::Write) {
        self.0.write(theme, cache, writer);
        writer.write_all(b"\n").unwrap();
        self.1.write(theme, cache, writer);
    }
}
