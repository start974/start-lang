use ariadne::{Cache, Config, IndexType, Label, ReportKind};
use location::{Located, Report, SourceId};
use pp::theme::Theme;

mod error;
mod errors;
mod message;
mod result_errors;

pub use error::Error;
pub use errors::Errors;
pub use message::Message;
pub use result_errors::ResultErrors;

// ===========================================================================
// Error trait
// ===========================================================================

#[deprecated()]
pub trait ErrorCode {
    /// error code
    fn code(&self) -> i32;
}

#[deprecated()]
pub trait ErrorPrint {
    /// print error on stderr
    fn eprint(&self, theme: &Theme, cache: &mut impl Cache<SourceId>) -> std::io::Result<()>;
}

#[deprecated()]
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
