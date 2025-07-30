use std::fmt::Display;

use super::location::{Located, SourceId};
use super::location::{Report, ReportBuilder};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use ariadne::{Cache, Config, IndexType, ReportKind};

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
    pub fn text(mut self, text: impl Display) -> Self {
        self.0.push(MessageKind::Text(text.to_string()));
        self
    }

    /// important message part
    pub fn important(mut self, text: impl Display) -> Self {
        self.0.push(MessageKind::Important(text.to_string()));
        self
    }

    /// import text quoted
    pub fn quoted(self, text: impl Display) -> Self {
        self.important(format!("\"{text}\""))
    }

    /// add message from pretty
    pub fn of_pretty(self, p: &impl Pretty) -> Self {
        self.text(p.make_string(&Theme::default()))
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

pub trait ErrorPrint {
    /// print error on stderr
    fn eprint(&self, theme: &Theme, cache: &mut impl Cache<SourceId>) -> std::io::Result<()>;
}

pub trait ErrorReport: ErrorCode + Located {
    /// finalize report
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a>;

    /// message of error
    fn message(&self) -> Message;

    /// report of error
    fn report(&self, theme: &Theme) -> Report {
        self.finalize(
            theme,
            Report::build(ReportKind::Error, self.loc().clone())
                .with_config(Config::default().with_index_type(IndexType::Byte))
                .with_code(self.code())
                .with_message(self.message().make_string(theme)),
        )
    }
}

impl<E> ErrorPrint for E
where
    E: ErrorReport,
{
    fn eprint(&self, theme: &Theme, cache: &mut impl Cache<SourceId>) -> std::io::Result<()> {
        self.report(theme).eprint(cache)
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

impl<E> ErrorPrint for Vec<E>
where
    E: ErrorPrint,
{
    fn eprint(&self, theme: &Theme, cache: &mut impl Cache<SourceId>) -> std::io::Result<()> {
        for (i, error) in self.iter().enumerate() {
            if i > 0 {
                eprintln!();
            }
            error.eprint(theme, cache)?;
        }
        Ok(())
    }
}
