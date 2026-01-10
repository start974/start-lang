use ariadne::{Cache, Config, IndexType, Label, ReportKind};
use location::{Located, Report, SourceId};
use pp::prelude::*;

// ===========================================================================
// Message
// ===========================================================================
mod message;
pub use message::Message;

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

// ===========================================================================
// Result extended
// ===========================================================================
pub trait ResultExt<T, E> {
    fn combine<U>(self, other: Result<U, Vec<E>>) -> Result<(T, U), Vec<E>>;
}

impl<T, E> ResultExt<T, E> for Result<T, Vec<E>> {
    fn combine<U>(self, other: Result<U, Vec<E>>) -> Result<(T, U), Vec<E>> {
        match (self, other) {
            (Ok(t), Ok(u)) => Ok((t, u)),
            (Err(mut e1), Err(e2)) => {
                e1.extend(e2);
                Err(e1)
            }
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}
