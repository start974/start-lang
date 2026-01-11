pub use colored::{Color, Styles};
use colored::{ColoredString,  Style};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColorInfo {
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    styles: Vec<Styles>,
}

impl ColorInfo {
    /// set foreground color
    pub fn fg_color<S: Into<Color>>(mut self, color: S) -> Self {
        self.fg_color = Some(color.into());
        self
    }

    /// set background color
    pub fn bg_color<S: Into<Color>>(mut self, color: S) -> Self {
        self.bg_color = Some(color.into());
        self
    }

    /// clear style
    pub fn clear(mut self) -> Self {
        self.styles.clear();
        self
    }

    /// bold style
    pub fn bold(mut self) -> Self {
        self.styles.push(Styles::Bold);
        self
    }

    /// underline style
    pub fn underline(mut self) -> Self {
        self.styles.push(Styles::Underline);
        self
    }

    /// italic style
    pub fn italic(mut self) -> Self {
        self.styles.push(Styles::Italic);
        self
    }

    /// dimmed style
    pub fn dimmed(mut self) -> Self {
        self.styles.push(Styles::Dimmed);
        self
    }

    /// blink style
    pub fn blink(mut self) -> Self {
        self.styles.push(Styles::Blink);
        self
    }

    /// hidden style
    pub fn hidden(mut self) -> Self {
        self.styles.push(Styles::Hidden);
        self
    }

    /// strikethrough style
    pub fn strikethrough(mut self) -> Self {
        self.styles.push(Styles::Strikethrough);
        self
    }

    /// color a string
    pub fn colorize(&self, s: &str) -> ColoredString {
        let mut cs = ColoredString::from(s);
        cs.fgcolor = self.fg_color;
        cs.bgcolor = self.bg_color;
        cs.style = Style::from_iter(self.styles.clone());
        cs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_info() {
        let ci = ColorInfo::default()
            .fg_color(Color::Red)
            .bg_color(Color::Black)
            .bold()
            .underline();
        let s = "Hello, World!";
        let colored_str = ci.colorize(s);
        assert_eq!(colored_str.input, s);
        assert_eq!(colored_str.fgcolor, Some(Color::Red));
        assert_eq!(colored_str.bgcolor, Some(Color::Black));
        assert!(colored_str.style.contains(Styles::Underline));
        assert!(colored_str.style.contains(Styles::Bold));
    }

    #[test]
    fn all_styles() {
        let ci = ColorInfo::default()
            .fg_color(Color::Green)
            .italic()
            .dimmed()
            .blink()
            .hidden()
            .strikethrough();
        let s = "Styled Text";
        let colored_str = ci.colorize(s);
        assert_eq!(colored_str.input, s);
        assert_eq!(colored_str.fgcolor, Some(Color::Green));
        dbg!("Styles: {:?}", colored_str.style);
        assert!(colored_str.style.contains(Styles::Italic));
        assert!(colored_str.style.contains(Styles::Dimmed));
        assert!(colored_str.style.contains(Styles::Blink));
        assert!(colored_str.style.contains(Styles::Hidden));
        assert!(colored_str.style.contains(Styles::Strikethrough));
    }

    #[test]
    fn test_clear_styles() {
        let ci = ColorInfo::default()
            .fg_color(Color::Blue)
            .bold()
            .underline()
            .clear();
        let s = "No Styles";
        let colored_str = ci.colorize(s);
        assert_eq!(colored_str.input, s);
        assert_eq!(colored_str.fgcolor, Some(Color::Blue));
        assert!(colored_str.style.contains(Styles::Clear));
    }
}
