use super::{Ident, NConst};
use colored::{Color, ColoredString, Colorize, Styles};
use lazy_static::lazy_static;
use pretty::{RcDoc, Render, RenderAnnotated};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColorInfo {
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    styles: Vec<Styles>,
}

impl ColorInfo {
    /// set fg color
    fn fg_color<S: Into<Color>>(mut self, color: S) -> Self {
        self.fg_color = Some(color.into());
        self
    }

    /// set bg color
    //fn bg_color<S: Into<Color>>(mut self, color: S) -> Self {
        //self.bg_color = Some(color.into());
        //self
    //}

    /// set style
    fn style(mut self, style: Styles) -> Self {
        self.styles.push(style);
        self
    }

    /// color a string
    fn colorize(&self, s: &str) -> ColoredString {
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

struct StreamColored<W> {
    upstream: W,
    color_info_stack: VecDeque<ColorInfo>,
}

impl<W> StreamColored<W> {
    fn new(upstream: W) -> Self {
        Self {
            color_info_stack: VecDeque::new(),
            upstream,
        }
    }
}

impl<W> Render for StreamColored<W>
where
    W: std::fmt::Write,
{
    type Error = std::fmt::Error;

    fn write_str_all(&mut self, s: &str) -> Result<(), Self::Error> {
        if let Some(color_info) = self.color_info_stack.front() {
            write!(self.upstream, "{}", color_info.colorize(s))
        } else {
            write!(self.upstream, "{}", s)
        }
    }

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        self.write_str_all(s).map(|_| s.len())
    }

    fn fail_doc(&self) -> Self::Error {
        std::fmt::Error
    }
}

impl<W> RenderAnnotated<'_, ColorInfo> for StreamColored<W>
where
    W: std::fmt::Write,
{
    fn push_annotation(&mut self, annot: &ColorInfo) -> Result<(), Self::Error> {
        self.color_info_stack.push_front(annot.clone());
        Ok(())
    }

    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        self.color_info_stack.pop_front();
        Ok(())
    }
}

pub struct Theme {
    /// limit to try to align
    width: usize,

    /// keyword color
    keyword: ColorInfo,
    /// operator color
    operator: ColorInfo,

    /// var definition color
    def_var: ColorInfo,

    /// expression var color
    expr_var: ColorInfo,
    /// constant color
    number: ColorInfo,

    /// type var
    ty_var: ColorInfo,
}

pub type Doc<'a> = RcDoc<'a, ColorInfo>;

impl Default for Theme {
    /// theme without color
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

impl Theme {
    /// pprint keyword
    pub fn keyword<'a>(&self, keyword: &'a str) -> Doc<'a> {
        Doc::text(keyword).annotate(self.keyword.clone())
    }

    /// pprint operator
    pub fn operator<'a>(&self, operator: &'a str) -> Doc<'a> {
        Doc::text(operator).annotate(self.operator.clone())
    }

    /// ppprint definition variable
    pub fn def_var<'a>(&self, var: &Ident) -> Doc<'a> {
        Doc::text(var.to_string()).annotate(self.def_var.clone())
    }

    /// pprint variable expression
    pub fn expr_var<'a>(&self, var: &Ident) -> Doc<'a> {
        Doc::text(var.to_string()).annotate(self.expr_var.clone())
    }

    /// pprint constant expression
    pub fn number<'a>(&self, n: &NConst) -> Doc<'a> {
        Doc::text(n.to_string()).annotate(self.number.clone())
    }

    /// pprint type variable
    pub fn ty_var<'a>(&self, var: &Ident) -> Doc<'a> {
        Doc::text(var.to_string()).annotate(self.ty_var.clone())
    }

    /// pprint keyword type
    pub fn kw_type<'a>(&self) -> Doc<'a> {
        self.keyword("type")
    }

    /// pprint keyword type
    pub fn kw_def<'a>(&self) -> Doc<'a> {
        self.keyword("def")
    }

    /// pprint operator eqdef
    pub fn op_eq_def<'a>(&self) -> Doc<'a> {
        self.operator(":=")
    }

    /// pprint opertor typed by ":"
    pub fn op_typed_by<'a>(&self) -> Doc<'a> {
        self.operator(":")
    }
}

lazy_static! {
    pub static ref THEME: Theme = Theme {
        width: 80,
        keyword: ColorInfo::default().fg_color(Color::Magenta),
        operator: ColorInfo::default().fg_color(Color::Red),
        def_var: ColorInfo::default().fg_color(Color::Blue),
        expr_var: ColorInfo::default()
            .fg_color(Color::Blue)
            .style(Styles::Bold),
        number: ColorInfo::default().fg_color(Color::Green),
        ty_var: ColorInfo::default().fg_color(Color::Yellow),
    };
    pub static ref THEME_NO_COLOR: Theme = Theme::default();
}

pub trait Pretty {
    /// pretty print
    fn pretty(&self, theme: &Theme) -> Doc<'_>;

    /// render to string
    fn to_string(&self) -> String {
        let theme = &THEME_NO_COLOR;
        let mut stream = StreamColored::new(String::new());
        self.pretty(theme)
            .render_raw(theme.width, &mut stream)
            .unwrap();
        stream.upstream
    }

    /// render to colored string
    fn to_string_colored(&self) -> String {
        let theme = &THEME;
        let mut stream = StreamColored::new(String::new());
        self.pretty(theme)
            .render_raw(theme.width, &mut stream)
            .unwrap();
        stream.upstream
    }
}
