use crate::message::Message;
use ariadne::{Cache, Config, IndexType, Label, ReportKind};
use ir::location::{Location, Report, SourceId, Span, Spanned, SpannedSet};
use pp::theme::Theme;

#[derive(Debug)]
pub struct Error {
    code: i32,
    header: Message,
    span: Span,
    text: Option<Message>,
    note: Option<Message>,
    /// how the pass corrected the error to go on
    recovery: Option<Message>,
    /// how the user can fix the error
    help: Option<Message>,
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
            recovery: None,
            help: None,
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

    pub fn recovery(&self) -> Option<&Message> {
        self.recovery.as_ref()
    }

    pub fn help(&self) -> Option<&Message> {
        self.help.as_ref()
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

    /// say how the pass corrected the error to go on
    pub fn with_recovery(mut self, recovery: Message) -> Self {
        if !recovery.is_nil() {
            self.recovery = Some(recovery);
        }
        self
    }

    /// advise the user how to fix the error
    pub fn with_help(mut self, help: Message) -> Self {
        if !help.is_nil() {
            self.help = Some(help);
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
        // one note, so that ariadne does not number them
        let recovery = self
            .recovery
            .as_ref()
            .map(|recovery| Message::text("(").append(recovery.clone()).with_text(")"));
        let note = match (&self.note, recovery) {
            (Some(note), Some(recovery)) => {
                Some(note.clone().with_line().with_line().append(recovery))
            }
            (note, recovery) => note.clone().or(recovery),
        };
        if let Some(note) = note {
            report_builder.add_note(note.make_string(&theme.error.note));
        }
        if let Some(help) = &self.help {
            report_builder.add_help(help.make_string(&theme.error.note));
        }
        report_builder.finish()
    }

    /// print error on stderr
    pub fn eprint(&self, id_source: &SourceId, theme: &Theme, cache: &mut impl Cache<SourceId>) {
        self.report(id_source, theme)
            .eprint(cache)
            .expect("cannot write the error on stderr");
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

// ============================================================================
// Test
// ============================================================================
// ============================================================================
// Errors
// ============================================================================

#[derive(Debug)]
pub struct Errors {
    source_id: SourceId,
    errs: Vec<Error>,
}

impl Errors {
    pub fn with_source_id(mut self, source_id: SourceId) -> Self {
        self.source_id = source_id;
        self
    }

    /// add error
    pub fn append(mut self, e: Error) -> Self {
        self.errs.push(e);
        self
    }

    /// combine two errors
    pub fn combine(mut self, mut other: Errors) -> Self {
        self.errs.append(&mut other.errs);
        self
    }

    /// errors in source order (stable: same place keeps the pass order)
    pub fn sorted(mut self) -> Self {
        self.errs
            .sort_by_key(|err| (err.span.start(), err.span.end()));
        self
    }

    /// print all errors on stderr
    pub fn eprint(&self, theme: &Theme, cache: &mut impl Cache<SourceId>) {
        for err in &self.errs {
            err.eprint(&self.source_id, theme, cache);
        }
    }

    /// number of errors
    pub fn len(&self) -> usize {
        self.errs.len()
    }

    /// no error
    pub fn is_empty(&self) -> bool {
        self.errs.is_empty()
    }

    /// exit code: the code of the only error, `1` for several, `0` for none
    pub fn code(&self) -> i32 {
        match self.errs.as_slice() {
            [] => 0,
            [err] => err.code(),
            _ => 1,
        }
    }
}

/// no error
impl Default for Errors {
    fn default() -> Self {
        Self {
            source_id: SourceId::Unknown,
            errs: Vec::new(),
        }
    }
}

impl From<Error> for Errors {
    fn from(e: Error) -> Self {
        Self {
            source_id: SourceId::Unknown,
            errs: vec![e],
        }
    }
}

impl FromIterator<Error> for Errors {
    fn from_iter<T: IntoIterator<Item = Error>>(iter: T) -> Self {
        Self {
            source_id: SourceId::Unknown,
            errs: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Errors {
    type Item = Error;
    type IntoIter = std::vec::IntoIter<Error>;

    fn into_iter(self) -> Self::IntoIter {
        self.errs.into_iter()
    }
}

impl Extend<Error> for Errors {
    fn extend<I: IntoIterator<Item = Error>>(&mut self, iter: I) {
        self.errs.extend(iter);
    }
}

impl<'a> IntoIterator for &'a Errors {
    type Item = &'a Error;
    type IntoIter = std::slice::Iter<'a, Error>;

    fn into_iter(self) -> Self::IntoIter {
        self.errs.iter()
    }
}

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
            .with_note(Message::text("This is a note."))
            .with_help(Message::text("This is a help."));
        assert_eq!(error.code(), 1001);
        assert_eq!(error.header().make_string(&theme), "Test error");
        assert_eq!(
            error.text().unwrap().make_string(&theme),
            "This is a test error message."
        );
        assert_eq!(error.note().unwrap().make_string(&theme), "This is a note.");
        assert_eq!(error.help().unwrap().make_string(&theme), "This is a help.");
        assert_eq!(error.span(), Span::new(0, 5));
    }

    #[test]
    fn report_generation() {
        let theme = Theme::default();
        let error = Error::new(1002, Message::text("Another error"))
            .with_span(Span::new(10, 20))
            .with_text(Message::text("Detailed error description."));
        let report = error.report(&SourceId::Unknown, &theme);
        let dbg_str = format!("{report:?}");
        assert!(dbg_str.contains("Another error"));
        assert!(dbg_str.contains("1002"));
    }

    #[test]
    fn singleton() {
        let err = Error::new(1001, Message::text("Header")).with_span(Span::new(0, 1));
        let errs = Errors::from(err);

        assert_eq!(errs.code(), 1001);
        assert_eq!(errs.len(), 1);
    }

    #[test]
    fn append() {
        let err1 = Error::new(1001, Message::text("Header1")).with_span(Span::new(0, 1));
        let err2 = Error::new(1002, Message::text("Header2")).with_span(Span::new(2, 3));
        let errs = Errors::from(err1).append(err2);
        assert_eq!(errs.code(), 1);
        assert_eq!(errs.len(), 2);
    }

    #[test]
    fn combine() {
        let err1 = Error::new(1001, Message::text("Header1")).with_span(Span::new(0, 1));
        let err2 = Error::new(1002, Message::text("Header2")).with_span(Span::new(2, 3));
        let errs1 = Errors::from(err1);
        let errs2 = Errors::from(err2);
        let combined = errs1.combine(errs2);

        assert_eq!(combined.code(), 1);
        assert_eq!(combined.len(), 2);
    }

    #[test]
    fn sorted() {
        let at = |code, start| {
            Error::new(code, Message::text("H")).with_span(Span::new(start, start + 1))
        };
        let errs: Errors = vec![at(3, 8), at(1, 2), at(2, 8), at(0, 0)]
            .into_iter()
            .collect();
        let codes: Vec<i32> = errs.sorted().into_iter().map(|err| err.code()).collect();
        assert_eq!(codes, vec![0, 1, 3, 2]);
    }

    #[test]
    fn source_id() {
        let err = Error::new(1001, Message::text("Header")).with_span(Span::new(0, 1));
        let errs = Errors::from(err);
        assert_eq!(errs.source_id, SourceId::Unknown);
        let errs = errs.with_source_id(SourceId::Repl);
        assert_eq!(errs.source_id, SourceId::Repl);
    }

    #[test]
    fn from_iter() {
        let err1 = Error::new(1001, Message::text("Header1")).with_span(Span::new(0, 1));
        let err2 = Error::new(1002, Message::text("Header2")).with_span(Span::new(2, 3));
        let errs: Errors = vec![err1, err2].into_iter().collect();

        assert_eq!(errs.code(), 1);
        assert_eq!(errs.len(), 2);

        let mut iter = errs.into_iter();
        assert_eq!(iter.next().unwrap().code(), 1001);
        assert_eq!(iter.next().unwrap().code(), 1002);
    }

    #[test]
    fn iterate_by_reference() {
        let errs: Errors = vec![
            Error::new(1001, Message::text("Header1")),
            Error::new(1002, Message::text("Header2")),
        ]
        .into_iter()
        .collect();
        let codes: Vec<_> = (&errs).into_iter().map(Error::code).collect();
        assert_eq!(codes, [1001, 1002]);
        assert_eq!(errs.len(), 2);
    }
}
