use super::parse_yml_theme::{parse_yml_keys, ThemeKey};
use crate::ast::{Ident, NConst};
use crate::error::*;
pub use colored::{Color, Styles};
use colored::{ColoredString, Colorize};
use lazy_static::lazy_static;
use pretty::RcDoc;
use std::sync::Mutex;

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
    static ref THEME: Mutex<Theme> = Mutex::new(Theme::default());
    static ref THEME_NO_COLOR: Theme = Theme::default();
}

/// set global theme
pub fn set_theme_from_yml(file_name: &str, content: String) -> Result<(), Errors> {
    for key in parse_yml_keys(file_name, content)? {
        match key {
            ThemeKey::Width(w) => THEME.lock().unwrap().width = w,
            ThemeKey::Keyword(c) => THEME.lock().unwrap().keyword = c,
            ThemeKey::Operator(c) => THEME.lock().unwrap().operator = c,
            ThemeKey::DefVar(c) => THEME.lock().unwrap().def_var = c,
            ThemeKey::ExprVar(c) => THEME.lock().unwrap().expr_var = c,
            ThemeKey::Number(c) => THEME.lock().unwrap().number = c,
            ThemeKey::TyVar(c) => THEME.lock().unwrap().ty_var = c,
        }
    }
    Ok(())
}

/// get global theme
pub fn get_theme() -> &'static Mutex<Theme> {
    &THEME
}

/// get no color theme
pub fn get_theme_no_color() -> &'static Theme {
    &THEME_NO_COLOR
}
