use crate::message::Message;
use ariadne::{Cache, Config, IndexType, Label, ReportKind};
use location::{Location, Report, SourceId, Span, Spanned, SpannedSet};
use pp::theme::Theme;

pub struct Error {
    code: i32,
    header: Message,
    span: Span,
    text: Option<Message>,
    note: Option<Message>,
}

impl Error {
    /// create new error
    pub fn new(code: i32, header: Message) -> Self {
        Self {
            code,
            header,
            span: Span::default(),
            text: None,
            note: None,
        }
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn header(&self) -> &Message {
        &self.header
    }

    pub fn text(&self) -> Option<&Message> {
        self.text.as_ref()
    }

    pub fn note(&self) -> Option<&Message> {
        self.note.as_ref()
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

    /// add span to error
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    /// report of error
    pub fn report<'a, 'id>(&self, id_source: &'a SourceId, theme: &Theme) -> Report<'a, 'id> {
        let loc = Location::new(id_source, self.span);
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
    pub fn eprint(&self, id_source: &SourceId, theme: &Theme, cache: &mut impl Cache<SourceId>) {
        self.report(id_source, theme).eprint(cache).unwrap();
    }
}

impl Spanned for Error {
    fn span(&self) -> Span {
        self.span
    }
}

impl SpannedSet for Error {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
