use crate::message::Message;
use ariadne::{Cache, Config, IndexType, Label, ReportKind};
use location::{GetSpan, Location, Report, SetSpan, SourceId, Span};
use pp::theme::Theme;

#[derive(Debug)]
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
        if !text.is_nil() {
            self.text = Some(text);
        }
        self
    }

    /// add note to error
    pub fn with_note(mut self, note: Message) -> Self {
        if !note.is_nil() {
            self.note = Some(note);
        }
        self
    }

    /// add span to error
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    /// report of error
    pub fn report<'a, 'id>(&self, id_source: &'a SourceId, theme: &Theme) -> Report<'a, 'id> {
        let config = Config::default().with_index_type(IndexType::Byte);
        let loc = Location::new(id_source, self.span);
        let mut report_builder = Report::build(ReportKind::Error, loc)
            .with_config(config)
            .with_code(self.code)
            .with_message(self.header.make_string(&theme.error.head));
        if let Some(text) = &self.text {
            let mut label = Label::new(loc).with_message(text.make_string(&theme.error.text));
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

impl GetSpan for Error {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Error {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

// ============================================================================
// Test
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Message;
    use pp::MessageTheme;

    #[test]
    fn error_report() {
        let theme = MessageTheme::default();
        let error = Error::new(1001, Message::text("Test error"))
            .with_span(Span::new(0, 5))
            .with_text(Message::text("This is a test error message."))
            .with_note(Message::text("This is a note."));
        assert_eq!(error.code(), 1001);
        assert_eq!(error.header().make_string(&theme), "Test error");
        assert_eq!(
            error.text().unwrap().make_string(&theme),
            "This is a test error message."
        );
        assert_eq!(error.note().unwrap().make_string(&theme), "This is a note.");
        assert_eq!(error.span(), Span::new(0, 5));
    }

    #[test]
    fn report_generation() {
        let theme = Theme::default();
        let error = Error::new(1002, Message::text("Another error"))
            .with_span(Span::new(10, 20))
            .with_text(Message::text("Detailed error description."));
        let report = error.report(&SourceId::Unknown, &theme);
        let dbg_str = format!("{:?}", report);
        eprintln!("{}", dbg_str);
        assert!(dbg_str.contains("Another error"));
        assert!(dbg_str.contains("1002"));
    }
}
