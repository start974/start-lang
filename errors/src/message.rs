use std::fmt::Display;

use pp::{
    pretty::{Pretty, StreamColored},
    theme::{Doc, MessageTheme, Theme},
};

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

    /// append doc
    pub fn append(mut self, doc: Self) -> Self {
        for kind in doc.0 {
            self.add_kind(kind);
        }
        self
    }

    /// append doc if condition is true
    pub fn append_if<F>(self, cond: bool, f_doc: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        if cond { self.append(f_doc()) } else { self }
    }

    pub fn append_opt(self, doc: Option<Self>) -> Self {
        if let Some(d) = doc {
            self.append(d)
        } else {
            self
        }
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

    /// with text message
    pub fn with_text(mut self, text: impl Display) -> Self {
        self.add_kind(MessageKind::Normal(text.to_string()));
        self
    }

    /// make text quoted
    pub fn quoted(text: impl Display) -> Self {
        Self::text(format!("\"{text}\""))
    }

    /// with quoted text message
    pub fn with_quoted(self, text: impl Display) -> Self {
        self.append(Self::quoted(text))
    }

    /// add new line in message
    pub fn line() -> Self {
        Self(vec![MessageKind::NewLine])
    }

    /// with line in message
    pub fn with_line(self) -> Self {
        self.append(Self::line())
    }

    /// add message from pretty
    pub fn of_pretty(p: &impl Pretty) -> Self {
        Self::text(p.make_string(&Theme::default()))
    }

    // with pretty
    pub fn with_pretty(self, p: &impl Pretty) -> Self {
        self.append(Self::of_pretty(p))
    }

    // make a doc normal
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
