use crate::{ColorInfo, Doc};

pub struct MessageTheme {
    /// limit to try to align
    pub width: usize,
    /// important color
    pub important: ColorInfo,
    /// normal color
    pub normal: ColorInfo,
}

impl Default for MessageTheme {
    fn default() -> Self {
        Self {
            width: 120,
            important: ColorInfo::default(),
            normal: ColorInfo::default(),
        }
    }
}

impl MessageTheme {
    /// pretty normal message
    pub fn normal<'a>(&self, text: &impl ToString) -> Doc<'a> {
        Doc::text(text.to_string()).annotate(self.normal.clone())
    }

    /// pretty important message
    pub fn important<'a>(&self, text: &impl ToString) -> Doc<'a> {
        Doc::text(text.to_string()).annotate(self.important.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_message_theme() {
        let message_theme = MessageTheme::default();
        assert_eq!(message_theme.width, 120);
    }
}
