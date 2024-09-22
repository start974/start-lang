use super::*;
use colored::{Color, ColoredString, Colorize, Styles};
use lazy_static::lazy_static;
use pretty::{RcDoc, Render, RenderAnnotated};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorInfo {
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    style: Styles,
}
impl ColorInfo {
    /// set fg color
    fn color<S: Into<Color>>(mut self, color: S) -> Self {
        self.fg_color = Some(color.into());
        self
    }

    /// set bg color
    fn on_color<S: Into<Color>>(mut self, color: S) -> Self {
        self.bg_color = Some(color.into());
        self
    }

    /// set style
    fn style(mut self, style: Styles) -> Self {
        self.style = style;
        self
    }

    /// color a string
    fn colorize(&self, s: &str) -> ColoredString {
        let mut cs = match self.style {
            Styles::Clear => s.clear(),
            Styles::Bold => s.bold(),
            Styles::Dimmed => s.dimmed(),
            Styles::Italic => s.italic(),
            Styles::Underline => s.underline(),
            Styles::Blink => s.blink(),
            Styles::Reversed => s.reverse(),
            Styles::Hidden => s.hidden(),
            Styles::Strikethrough => s.strikethrough(),
        };
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
        if let Some(color_info) = self.color_info_stack.get(0) {
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
impl Default for ColorInfo {
    fn default() -> Self {
        Self {
            fg_color: None,
            bg_color: None,
            style: Styles::Clear,
        }
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
    /// type color
    ty: ColorInfo,
}

type Doc<'a> = RcDoc<'a, ColorInfo>;

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
            ty: ColorInfo::default(),
        }
    }
}

impl Theme {
    /// format keyword
    pub fn keyword<'a>(&self, keyword: &'a str) -> Doc<'a> {
        Doc::text(keyword).annotate(self.keyword)
    }

    /// format operator
    pub fn operator<'a>(&self, operator: &'a str) -> Doc<'a> {
        Doc::text(operator).annotate(self.operator)
    }

    /// format definition variable
    pub fn def_var(&self, var: &Ident) -> Doc<'_> {
        Doc::text(var.to_string()).annotate(self.def_var)
    }

    /// format variable expression
    pub fn expr_var(&self, var: &Ident) -> Doc<'_> {
        Doc::text(var.to_string()).annotate(self.expr_var)
    }

    /// format constant expression
    pub fn number(&self, constant: &Constant) -> Doc<'_> {
        match constant {
            Constant::N(c) => Doc::text(c.to_string()).annotate(self.number),
        }
    }
}

lazy_static! {
    pub static ref THEME: Theme = Theme {
        width: 80,
        keyword: ColorInfo::default().color(Color::Magenta),
        operator: ColorInfo::default().color(Color::Red),
        def_var: ColorInfo::default().color(Color::Blue),
        expr_var: ColorInfo::default().color(Color::Blue).style(Styles::Bold),
        number: ColorInfo::default().color(Color::Green),
        ty: ColorInfo::default().color(Color::Yellow),
    };
}

trait Pretty {
    /// pretty print
    fn pretty(&self, theme: &Theme) -> Doc<'_>;
}

impl std::fmt::Display for dyn Pretty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let theme = &THEME;
        let mut stream = StreamColored::new(f);
        self.pretty(theme).render_raw(theme.width, &mut stream)
    }
}
