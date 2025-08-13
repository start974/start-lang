use super::location::Report;
use super::location::{Located, SourceId};
use super::pretty::StreamColored;
use super::theme::MessageTheme;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use ariadne::{Cache, Config, IndexType, Label, ReportKind};
use std::fmt::Display;

// ===========================================================================
// Message
// ===========================================================================
#[derive(Debug, Clone)]
enum MessageKind {
    Normal(String),
    Important(String),
    NewLine,
}

#[derive(Debug, Clone)]
pub struct Message(Vec<MessageKind>);
impl Message {
    /// empty message
    pub fn nil() -> Self {
        Self(Vec::new())
    }

    fn add_kind(&mut self, kind: MessageKind) {
        match (&kind, self.0.last_mut()) {
            (MessageKind::Normal(text), Some(MessageKind::Normal(last))) => last.push_str(text),
            (MessageKind::Important(text), Some(MessageKind::Important(last))) => {
                last.push_str(text)
            }
            (_, _) => self.0.push(kind.clone()),
        }
    }

    /// add doc to message
    pub fn extend(&mut self, doc: Self) {
        for kind in doc.0 {
            self.add_kind(kind);
        }
    }

    /// append doc
    pub fn append(mut self, doc: Self) -> Self {
        self.extend(doc);
        self
    }

    /// intersperse document
    pub fn intersperse<I>(docs: I, sep: Self) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let mut add_sep = false;
        let mut doc = Self::nil();
        for cur_doc in docs.into_iter() {
            if add_sep {
                doc = doc.append(sep.clone());
            } else {
                add_sep = true;
            }
            doc = doc.append(cur_doc);
        }
        doc
    }

    /// text message by default is normal
    pub fn text(text: impl Display) -> Self {
        Self(vec![MessageKind::Normal(text.to_string())])
    }

    /// add text to message
    pub fn add_text(&mut self, text: impl Display) {
        self.add_kind(MessageKind::Normal(text.to_string()));
    }

    /// with text message
    pub fn with_text(mut self, text: impl Display) -> Self {
        self.add_text(text);
        self
    }

    /// make text quoted
    pub fn quoted(text: impl Display) -> Self {
        Self::text(format!("\"{text}\""))
    }

    /// add quoted text to message
    pub fn add_quoted(&mut self, text: impl Display) {
        self.extend(Self::quoted(text));
    }

    /// with quoted text message
    pub fn with_quoted(mut self, text: impl Display) -> Self {
        self.add_quoted(text);
        self
    }

    /// add new line in message
    pub fn line() -> Self {
        Self(vec![MessageKind::NewLine])
    }

    /// add line in message
    pub fn add_line(&mut self) {
        self.extend(Self::line());
    }

    /// with line in message
    pub fn with_line(mut self) -> Self {
        self.add_line();
        self
    }

    /// add message from pretty
    pub fn of_pretty(p: &impl Pretty) -> Self {
        Self::text(p.make_string(&Theme::default()))
    }

    /*
    /// add pretty
    pub fn add_pretty(&mut self, p: &impl Pretty) {
        self.extend(Self::of_pretty(p));
    }

    /// with pretty
    pub fn with_pretty(mut self, p: &impl Pretty) -> Self {
        self.add_pretty(p);
        self
    }

    /// make a doc normal
    pub fn normal(mut self) -> Self {
        self.0 = self
            .0
            .iter()
            .map(|kind| match kind {
                MessageKind::Normal(text) => MessageKind::Normal(text.clone()),
                MessageKind::Important(text) => MessageKind::Normal(text.clone()),
                MessageKind::NewLine => MessageKind::NewLine,
            })
            .collect();
        self
    }
    */

    /// important message part
    pub fn important(mut self) -> Self {
        self.0 = self
            .0
            .iter()
            .map(|kind| match kind {
                MessageKind::Normal(text) => MessageKind::Important(text.clone()),
                MessageKind::Important(text) => MessageKind::Important(text.clone()),
                MessageKind::NewLine => MessageKind::NewLine,
            })
            .collect();
        self
    }

    pub fn pretty(&self, theme: &MessageTheme) -> Doc<'_> {
        Doc::intersperse(
            self.0.iter().map(|kind| match kind {
                MessageKind::Normal(text) => theme.normal(text),
                MessageKind::Important(text) => theme.important(text),
                MessageKind::NewLine => Doc::hardline(),
            }),
            Doc::nil(),
        )
    }

    pub fn make_string(&self, theme: &MessageTheme) -> String {
        let mut buffer = String::new();
        let mut stream = StreamColored::new(&mut buffer);
        let _ = self.pretty(theme).render_raw(theme.width, &mut stream);
        buffer
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
    fn report(&self, theme: &Theme) -> Report<'_> {
        let loc = self.loc();
        let mut report_builder = Report::build(ReportKind::Error, loc.clone())
            .with_config(Config::default().with_index_type(IndexType::Byte))
            .with_code(self.code())
            .with_message(self.head().make_string(&theme.error.head));
        if let Some(text) = self.text() {
            let mut label =
                Label::new(loc.clone()).with_message(text.make_string(&theme.error.text));
            if let Some(color) = theme.error.label_color() {
                label = label.with_color(*color);
            }
            report_builder.add_label(label)
        }
        if let Some(note) = self.note() {
            report_builder.add_note(note.make_string(&theme.error.note));
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
