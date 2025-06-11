pub use colored::{Color, Styles};
use colored::{ColoredString, Colorize};
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

    /// set bg color
    pub fn bg_color<S: Into<Color>>(mut self, color: S) -> Self {
        self.bg_color = Some(color.into());
        self
    }

    // set style
    /*    pub fn style(mut self, style: Styles) -> Self {*/
    /*self.styles.push(style);*/
    /*self*/
    /*}*/

    /// add many styles
    pub fn styles(mut self, styles: Vec<Styles>) -> Self {
        self.styles.extend(styles);
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
    /// constant color
    pub number: ColorInfo,

    /// type var
    pub ty_var: ColorInfo,
}

pub type Doc<'a> = RcDoc<'a, ColorInfo>;

impl Theme {
    /// create a default theme
    pub fn default_theme() -> Self {
        Self {
            width: 80,
            keyword: ColorInfo {
                fg_color: Some(Color::Magenta),
                bg_color: None,
                styles: vec![],
            },
            operator: ColorInfo {
                fg_color: Some(Color::Red),
                bg_color: None,
                styles: vec![],
            },
            def_var: ColorInfo {
                fg_color: Some(Color::Blue),
                bg_color: None,
                styles: vec![Styles::Bold],
            },
            expr_var: ColorInfo {
                fg_color: Some(Color::Blue),
                bg_color: None,
                styles: vec![],
            },
            number: ColorInfo {
                fg_color: Some(Color::Green),
                bg_color: None,
                styles: vec![],
            },
            ty_var: ColorInfo {
                fg_color: Some(Color::Yellow),
                bg_color: None,
                styles: vec![Styles::Italic],
            },
        }
    }

    /// print title
    pub fn title1<'a>(&self, title: &impl ToString) -> Doc<'a> {
        Doc::text(title.to_string()).annotate(ColorInfo {
            fg_color: Some(Color::Cyan),
            bg_color: None,
            styles: vec![Styles::Bold],
        })
    }

    /// error message
    pub fn error_message<'a>(&self, message: &impl ToString) -> Doc<'a> {
        Doc::text(message.to_string()).annotate(ColorInfo {
            fg_color: Some(Color::Red),
            bg_color: None,
            styles: vec![],
        })
    }

    /// error important message
    pub fn error_important<'a>(&self, message: &impl ToString) -> Doc<'a> {
        Doc::text(message.to_string()).annotate(ColorInfo {
            fg_color: Some(Color::Red),
            bg_color: None,
            styles: vec![Styles::Bold],
        })
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
    pub fn number<'a>(&self, n: &impl ToString) -> Doc<'a> {
        Doc::text(n.to_string()).annotate(self.number.clone())
    }

    /// pprint type variable
    pub fn ty_var<'a>(&self, var: &impl ToString) -> Doc<'a> {
        Doc::text(var.to_string()).annotate(self.ty_var.clone())
    }

    /// pprint keyword type
    pub fn kw_type<'a>(&self) -> Doc<'a> {
        self.keyword(&"type")
    }

    /// pprint keyword type
    pub fn kw_def<'a>(&self) -> Doc<'a> {
        self.keyword(&"def")
    }

    /// pprint operator eqdef
    pub fn op_eq_def<'a>(&self) -> Doc<'a> {
        self.operator(&":=")
    }

    /// pprint opertor typed by ":"
    pub fn op_typed_by<'a>(&self) -> Doc<'a> {
        self.operator(&":")
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            width: 80,
            keyword: ColorInfo::default(),
            operator: ColorInfo::default(),
            def_var: ColorInfo::default(),
            expr_var: ColorInfo::default(),
            number: ColorInfo::default(),
            ty_var: ColorInfo::default(),
        }
    }
}

impl AsRef<Theme> for &Theme {
    fn as_ref(&self) -> &Theme {
        self
    }
}

pub trait ThemeGet {
    /// get theme
    fn theme(&self) -> &Theme;
}
