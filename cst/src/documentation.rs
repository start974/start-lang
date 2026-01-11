use pp::prelude::*;

#[derive(Debug, Clone)]
pub struct Documentation {
    /// documentation lines
    doc: Vec<String>,
}

impl From<Vec<String>> for Documentation {
    fn from(doc: Vec<String>) -> Self {
        Self { doc }
    }
}

impl std::fmt::Display for Documentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.doc {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

impl Pretty for Documentation {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.doc.iter().map(|txt| theme.documentation(txt)),
            Doc::hardline(),
        )
        .group()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documentation_display() {
        let doc = Documentation::from(vec![
            "This is line 1.".to_string(),
            "This is line 2.".to_string(),
        ]);
        let expected = "This is line 1.\nThis is line 2.\n";
        assert_eq!(format!("{}", doc), expected);
    }

    #[test]
    fn documentation_pretty() {
        let doc = Documentation::from(vec![
            "This is line 1.".to_string(),
            "This is line 2.".to_string(),
        ]);
        let theme = Theme::default();
        let pp_str = doc.make_string(&theme);
        let expected = "This is line 1.\nThis is line 2.";
        assert_eq!(pp_str, expected);
    }
}
