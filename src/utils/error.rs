use super::location::Report;
use super::location::{Located, SourceId};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use ariadne::{Cache, Config, IndexType, Label, ReportKind};
use std::fmt::Display;

// ===========================================================================
// Message
// ===========================================================================
#[derive(Debug, Clone)]
enum MessageKind {
    Text(String),
    Important(String),
}

#[derive(Debug, Clone)]
pub struct Message(Vec<MessageKind>);
impl Message {
    /// empty message
    pub fn nil() -> Self {
        Self(Vec::new())
    }

    /// append doc
    pub fn append(mut self, doc: Self) -> Self {
        self.0.extend(doc.0);
        self
    }

    /// intersperse document
    pub fn intersperse<I>(docs: I, sep: Self) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let mut add_doc = false;
        let mut doc = Self::nil();
        for cur_doc in docs.into_iter() {
            doc = doc.append(cur_doc);
            if add_doc {
                doc = doc.append(sep.clone());
            } else {
                add_doc = true;
            }
        }
        doc
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
    /// head message of error
    fn head(&self) -> Message;

    /// label of error
    fn text(&self) -> Option<Message> {
        None
    }

    /// note of error
    fn note(&self) -> Option<Message> {
        None
    }

    /// report of error
    fn report(&self, theme: &Theme) -> Report {
        let loc = self.loc();
        let mut report_builder = Report::build(ReportKind::Error, loc.clone())
            .with_config(Config::default().with_index_type(IndexType::Byte))
            .with_code(self.code())
            .with_message(self.head().make_string(theme));
        if let Some(text) = self.text() {
            report_builder.add_label(Label::new(loc.clone()).with_message(text.make_string(theme)));
        }
        report_builder.finish()
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
    fn text(&self) -> Option<Message> {
        self.as_ref().text()
    }

    fn head(&self) -> Message {
        self.as_ref().head()
    }

    fn note(&self) -> Option<Message> {
        self.as_ref().note()
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
