pub use colored::{Color, Styles};
use colored::{ColoredString, Colorize};
use num_bigint::BigUint;
use pretty::RcDoc;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColorInfo {
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    styles: Vec<Styles>,
}

impl ColorInfo {
    /// set fg color
    pub fn fg_color<S: Into<Color>>(mut self, color: S) -> Self {
        self.fg_color = Some(color.into());
        self
    }

    /*
    /// set bg color
    pub fn bg_color<S: Into<Color>>(mut self, color: S) -> Self {
        self.bg_color = Some(color.into());
        self
    }
    */

    // set style
    pub fn style(mut self, style: Styles) -> Self {
        self.styles.push(style);
        self
    }

    /// color a string
    pub fn colorize(&self, s: &str) -> ColoredString {
        let mut cs = s.normal();
        for style in self.styles.iter() {
            cs = match style {
                Styles::Clear => s.clear(),
                Styles::Bold => s.bold(),
                Styles::Dimmed => s.dimmed(),
                Styles::Italic => s.italic(),
                Styles::Underline => s.underline(),
                Styles::Blink => s.blink(),
                Styles::Reversed => panic!("deprecated"),
                Styles::Hidden => s.hidden(),
                Styles::Strikethrough => s.strikethrough(),
            };
        }
        if let Some(c) = self.fg_color {
            cs = cs.color(c)
        }
        if let Some(c) = self.bg_color {
            cs = cs.on_color(c)
        }
        cs
    }
}

pub fn to_ariadne_color(color: Color) -> ariadne::Color {
    match color {
        Color::Black => ariadne::Color::Black,
        Color::Red => ariadne::Color::Red,
        Color::Green => ariadne::Color::Green,
        Color::Yellow => ariadne::Color::Yellow,
        Color::Blue => ariadne::Color::Blue,
        Color::Magenta => ariadne::Color::Magenta,
        Color::Cyan => ariadne::Color::Cyan,
        Color::White => ariadne::Color::White,
        Color::BrightBlack => ariadne::Color::BrightBlack,
        Color::BrightRed => ariadne::Color::BrightRed,
        Color::BrightGreen => ariadne::Color::BrightGreen,
        Color::BrightYellow => ariadne::Color::BrightYellow,
        Color::BrightBlue => ariadne::Color::BrightBlue,
        Color::BrightMagenta => ariadne::Color::BrightMagenta,
        Color::BrightCyan => ariadne::Color::BrightCyan,
        Color::BrightWhite => ariadne::Color::BrightWhite,
        Color::TrueColor { r, g, b } => ariadne::Color::Rgb(r, g, b),
    }
}

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

#[derive(Default)]
pub struct ErrorTheme {
    /// error message
    pub head: MessageTheme,
    /// text message
    pub text: MessageTheme,
    /// info message
    pub note: MessageTheme,
    /// label color
    pub label_color: Option<Color>,
}

impl ErrorTheme {
    /// get label color
    pub fn label_color(&self) -> Option<ariadne::Color> {
        self.label_color.map(to_ariadne_color)
    }
}

pub struct Theme {
    /// limit to try to align
    pub width: usize,
    /// keyword color
    pub keyword: ColorInfo,
    /// operator color
    pub operator: ColorInfo,
    /// var definition color
    pub def_var: ColorInfo,
    /// expression var color
    pub expr_var: ColorInfo,
    /// character color
    pub character: ColorInfo,
    /// number
    pub number: ColorInfo,
    /// boolean
    pub boolean: ColorInfo,
    /// type var
    pub ty_var: ColorInfo,
    /// comment
    pub comment: ColorInfo,
    /// documentation color
    pub documentation: ColorInfo,

    /// error theme
    pub error: ErrorTheme,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            width: 80,
            keyword: ColorInfo::default(),
            operator: ColorInfo::default(),
            def_var: ColorInfo::default(),
            expr_var: ColorInfo::default(),
            character: ColorInfo::default(),
            boolean: ColorInfo::default(),
            number: ColorInfo::default(),
            ty_var: ColorInfo::default(),
            comment: ColorInfo::default(),
            documentation: ColorInfo::default(),
            error: ErrorTheme::default(),
        }
    }
}

pub type Doc<'a> = RcDoc<'a, ColorInfo>;

impl Theme {
    /// create a default theme
    pub fn default_theme() -> Self {
        Self {
            width: 80,
            keyword: ColorInfo::default().fg_color(Color::Magenta),
            operator: ColorInfo::default().fg_color(Color::Red),
            def_var: ColorInfo::default()
                .fg_color(Color::Blue)
                .style(Styles::Bold),
            expr_var: ColorInfo::default().fg_color(Color::Blue),
            character: ColorInfo::default().fg_color(Color::Green),
            number: ColorInfo::default().fg_color(Color::Green),
            boolean: ColorInfo::default().fg_color(Color::Green),
            ty_var: ColorInfo::default()
                .fg_color(Color::Yellow)
                .style(Styles::Italic),
            comment: ColorInfo::default()
                .fg_color(Color::BrightBlack)
                .style(Styles::Italic),
            documentation: ColorInfo::default()
                .fg_color(Color::White)
                .style(Styles::Italic),
            error: ErrorTheme {
                head: MessageTheme {
                    width: 120,
                    important: ColorInfo::default()
                        .fg_color(Color::Red)
                        .style(Styles::Bold),
                    normal: ColorInfo::default().fg_color(Color::Red),
                },
                text: MessageTheme {
                    width: 120,
                    important: ColorInfo::default()
                        .fg_color(Color::Red)
                        .style(Styles::Bold),
                    normal: ColorInfo::default(),
                },
                note: MessageTheme {
                    width: 120,
                    important: ColorInfo::default()
                        .fg_color(Color::Yellow)
                        .style(Styles::Bold),
                    normal: ColorInfo::default().fg_color(Color::Yellow),
                },
                label_color: Some(Color::Red),
            },
        }
    }

    /// pprint keyword
    pub fn keyword<'a>(&self, keyword: &impl ToString) -> Doc<'a> {
        Doc::text(keyword.to_string()).annotate(self.keyword.clone())
    }

    /// pprint operator
    pub fn operator<'a>(&self, operator: &impl ToString) -> Doc<'a> {
        Doc::text(operator.to_string()).annotate(self.operator.clone())
    }

    /// ppprint definition variable
    pub fn def_var<'a>(&self, var: &impl ToString) -> Doc<'a> {
        Doc::text(var.to_string()).annotate(self.def_var.clone())
    }

    /// pprint variable expression
    pub fn expr_var<'a>(&self, var: &impl ToString) -> Doc<'a> {
        Doc::text(var.to_string()).annotate(self.expr_var.clone())
    }

    /// pprint constant expression
    pub fn character<'a>(&self, c: char) -> Doc<'a> {
        let c_escaped = match c {
            '\'' => "\\'",
            '\\' => "\\\\",
            '\n' => "\\n",
            '\r' => "\\r",
            '\t' => "\\t",
            _ => &c.to_string(),
        };
        Doc::text(format!("'{c_escaped}'")).annotate(self.character.clone())
    }

    /// pretty print number
    pub fn number<'a>(&self, n: &BigUint) -> Doc<'a> {
        let number_str: String = {
            let s = n.to_string();
            let mut res = String::new();
            for (i, c) in s.chars().rev().enumerate() {
                if i > 0 && i % 3 == 0 {
                    res.push('_');
                }
                res.push(c);
            }
            res.chars().rev().collect()
        };
        Doc::text(number_str).annotate(self.number.clone())
    }

    /// pretty print boolean
    pub fn boolean<'a>(&self, b: bool) -> Doc<'a> {
        Doc::text(if b { "true" } else { "false" }).annotate(self.boolean.clone())
    }

    /// pprint type variable
    pub fn ty_var<'a>(&self, var: &impl ToString) -> Doc<'a> {
        Doc::text(var.to_string()).annotate(self.ty_var.clone())
    }

    /// pprint comment color
    pub fn comment<'a>(&self, comment: &impl ToString) -> Doc<'a> {
        Doc::text(comment.to_string()).annotate(self.comment.clone())
    }

    pub fn documentation<'a>(&self, doc: &impl ToString) -> Doc<'a> {
        Doc::text(doc.to_string()).annotate(self.documentation.clone())
    }
}
