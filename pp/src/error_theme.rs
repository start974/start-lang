use crate::MessageTheme;

#[derive(Default)]
pub struct ErrorTheme {
    /// error message
    pub head: MessageTheme,
    /// text message
    pub text: MessageTheme,
    /// info message
    pub note: MessageTheme,
    /// label color
    pub label_color: Option<ariadne::Color>,
}

impl ErrorTheme {
    /// get label color
    pub fn label_color(&self) -> &Option<ariadne::Color> {
        &self.label_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_error_theme() {
        let error_theme = ErrorTheme::default();
        assert_eq!(error_theme.head.width, 120);
        assert_eq!(error_theme.text.width, 120);
        assert_eq!(error_theme.note.width, 120);
        assert_eq!(error_theme.label_color, None);
    }
}

