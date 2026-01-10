use crate::message::Message;
use ariadne::{Cache, Config, IndexType, Label, ReportKind};
use location::{Located, Location, Report, SourceId};
use pp::theme::Theme;

pub struct Error {
    code: i32,
    header: Message,
    location: Location,
    text: Option<Message>,
    note: Option<Message>,
}

impl Error {
    /// create new error
    pub fn new(code: i32, header: Message) -> Self {
        Self {
            code,
            header,
            location: Location::unknown(),
            text: None,
            note: None,
        }
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    /// add location to error
    pub fn with_location(mut self, loc: Location) -> Self {
        self.location = loc;
        self
    }

    /// add text to error
    pub fn with_text(mut self, text: Message) -> Self {
        self.text = Some(text);
        self
    }

    /// add note to error
    pub fn with_note(mut self, note: Message) -> Self {
        self.note = Some(note);
        self
    }

    /// report of error
    pub fn report(&self, theme: &Theme) -> Report<'_> {
        let loc = self.location.clone();
        let mut report_builder = Report::build(ReportKind::Error, loc.clone())
            .with_config(Config::default().with_index_type(IndexType::Byte))
            .with_code(self.code)
            .with_message(self.header.make_string(&theme.error.head));
        if let Some(text) = &self.text {
            let mut label =
                Label::new(loc.clone()).with_message(text.make_string(&theme.error.text));
            if let Some(color) = theme.error.label_color() {
                label = label.with_color(*color);
            }
            report_builder.add_label(label)
        }
        if let Some(note) = &self.note {
            report_builder.add_note(note.make_string(&theme.error.note));
        }
        report_builder.finish()
    }

    /// print error on stderr
    pub fn eprint(&self, theme: &Theme, cache: &mut impl Cache<SourceId>) {
        self.report(theme).eprint(cache).unwrap();
    }
}

impl Located for Error {
    fn loc(&self) -> Location {
        self.location.clone()
    }
}
