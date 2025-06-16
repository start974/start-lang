use super::super::super::ast::Identifier;
use crate::utils::pretty::Pretty;
use crate::utils::theme::Doc;
use crate::utils::theme::Theme;

enum TemplatePart {
    Text(String),
    Var(Identifier),
}

pub struct Template {
    parts: Vec<TemplatePart>,
}

impl Template {
    /// add a new template with the given parts
    pub fn new(parts: Vec<TemplatePart>) -> Self {
        Template { parts }
    }

    /// add a new identifier to the template
    pub fn add_identifier(&mut self, identifier: Identifier) {
        self.parts.push(TemplatePart::Var(identifier));
    }

    /// add a new text part to the template
    pub fn add_text(&mut self, text: String) {
        if let Some(TemplatePart::Text(last)) = self.parts.last_mut() {
            last.push_str(&text);
        } else {
            self.parts.push(TemplatePart::Text(text));
        }
    }
}

impl Pretty for Template {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let mut doc = Doc::nil();
        for part in &self.parts {
            match part {
                TemplatePart::Text(text) => {
                    doc = doc.append(Doc::text(text));
                }
                TemplatePart::Var(var) => {
                    doc = doc.append(Doc::group(
                        Doc::nil()
                            .append(Doc::text("{"))
                            .append(theme.expr_var(&var))
                            .append(Doc::text("}")),
                    ));
                }
            }
        }
        doc
    }
}
