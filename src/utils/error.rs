use super::location::{Located, SourceCache};
use super::location::{Report, ReportBuilder};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use crate::utils::writer::StringPrettyWriter;

// ===========================================================================
// Error Writer
// ===========================================================================

pub struct ErrorWriter<C, W, T> {
    cache: C,
    writer: W,
    theme: T,
}

impl<C, W, T> ErrorWriter<C, W, T> {
    pub fn new(theme: T, writer: W, cache: C) -> Self {
        Self {
            cache,
            theme,
            writer,
        }
    }
}
impl<C, W, T> ErrorWriter<C, W, T>
where
    W: std::io::Write,
    T: AsRef<Theme>,
    C: AsMut<SourceCache>,
{
    /// print error with writer
    pub fn eprint(&mut self, e: &impl ErrorWrite) {
        e.write(self.theme.as_ref(), self.cache.as_mut(), &mut self.writer);
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

    /// add message from pretty
    pub fn from_pretty(self, p: &impl Pretty) -> Self {
        let theme = Theme::default();
        let mut writer = StringPrettyWriter::make(&theme);
        writer.print(p);
        self.text(writer.writer_mut().get_str())
    }

    /// string of message
    pub fn to_string(&self, theme: &Theme) -> String {
        let mut writer = StringPrettyWriter::make(&theme);
        writer.print(self);
        writer.writer_mut().get_str().to_string()
    }
}

impl Pretty for Message {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.0.iter().map(|part| match part {
                MessageKind::Text(text) => theme.error_message(text),
                MessageKind::Important(text) => theme.error_important(text),
            }),
            Doc::nil(),
        )
    }
}

// ===========================================================================
// Error trait
// ===========================================================================

pub trait ErrorCode {
    /// error code
    fn code(&self) -> i32;
}

pub trait ErrorWrite {
    /// write error
    fn write(&self, theme: &Theme, cache: &mut SourceCache, writer: &mut dyn std::io::Write);
}

pub trait Error: ErrorCode + ErrorWrite {}

pub trait ErrorReport: ErrorCode + Located {
    /// finalize report
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a>;

    /// message of error
    fn message(&self) -> Message;

    /// report of error
    fn report(&self, theme: &Theme) -> Report {
        self.finalize(
            theme,
            Report::build(ariadne::ReportKind::Error, self.loc().clone())
                .with_code(self.code())
                .with_message(self.message().to_string(theme)),
        )
    }
}

impl<T> ErrorWrite for T
where
    T: ErrorReport,
{
    fn write(&self, theme: &Theme, cache: &mut SourceCache, writer: &mut dyn std::io::Write) {
        self.report(theme).write(cache, writer).unwrap(); // Use unwrap or proper error handling
    }
}

// ===========================================================================
// Error Box
// ===========================================================================
impl<E> ErrorCode for Box<E>
where
    E: ErrorCode,
{
    fn code(&self) -> i32 {
        self.as_ref().code()
    }
}

impl<E> ErrorReport for Box<E>
where
    E: ErrorReport,
{
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        self.as_ref().finalize(theme, report)
    }

    fn message(&self) -> Message {
        self.as_ref().message()
    }
}

impl<E> Error for Box<E> where E: ErrorReport + ErrorCode {}

// ===========================================================================
// Error Pair
// ===========================================================================

impl<E1, E2> ErrorCode for (E1, E2)
where
    E1: ErrorCode,
    E2: ErrorCode,
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

impl<E1, E2> ErrorWrite for (E1, E2)
where
    E1: ErrorWrite,
    E2: ErrorWrite,
{
    fn write(&self, theme: &Theme, cache: &mut SourceCache, writer: &mut dyn std::io::Write) {
        self.0.write(theme, cache, writer);
        writer.write_all(b"\n").unwrap();
        self.1.write(theme, cache, writer);
    }
}

impl<E1, E2> Error for (E1, E2)
where
    E1: ErrorReport + ErrorCode,
    E2: ErrorReport + ErrorCode,
{
}

// ===========================================================================
// Error Vec
// ===========================================================================
impl<E> ErrorCode for Vec<E>
where
    E: ErrorCode,
{
    fn code(&self) -> i32 {
        if self.is_empty() {
            0
        } else if self.len() == 1 {
            self[0].code()
        } else {
            1
        }
    }
}

impl<E> ErrorWrite for Vec<E>
where
    E: ErrorWrite,
{
    fn write(&self, theme: &Theme, cache: &mut SourceCache, writer: &mut dyn std::io::Write) {
        for (i, error) in self.iter().enumerate() {
            if i > 0 {
                writer.write_all(b"\n").unwrap();
            }
            error.write(theme, cache, writer);
        }
    }
}

impl<E> Error for Vec<E> where E: ErrorReport + ErrorCode {}
