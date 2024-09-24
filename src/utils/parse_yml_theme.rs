use super::theme::*;
use super::FResult;
use crate::error::*;
use crate::location::{Located, Location, Position};
use yaml_rust::{yaml, Yaml, YamlLoader};

const ERROR_PARSE_THEME_FILE: i32 = 105;
const ERROR_PARSE_COLOR: i32 = 106;
const ERROR_PARSE_STYLE: i32 = 106;

enum ColorInfoKey {
    Fg(Color),
    Bg(Color),
    Styles(Vec<Styles>),
}

pub enum ThemeKey {
    Width(usize),
    Keyword(ColorInfo),
    Operator(ColorInfo),
    DefVar(ColorInfo),
    ExprVar(ColorInfo),
    Number(ColorInfo),
    TyVar(ColorInfo),
}

fn parse_yml(file_name: &str, content: String) -> Result<Vec<Yaml>, Errors> {
    YamlLoader::load_from_str(&content).map_err(|scan_err| {
        let marker = scan_err.marker();
        let row = marker.line();
        let col = marker.col() + 1;
        let pos_start = Position::make(row, col);
        let pos_end = Position::make(row, col + 1);
        let content_vec: Vec<String> = content.split('\n').map(|s| s.to_string()).collect();
        let location = Location::make(file_name.to_string(), &content_vec, pos_start, pos_end);
        let msg = Head::new().text({
            use std::error::Error;
            #[allow(deprecated)]
            scan_err.description()
        });
        let err = Error::make(msg, ERROR_PARSE_THEME_FILE).set_location(location);
        Errors::from(err)
    })
}

fn error<T>(e: Error) -> FResult<(), T, Error> {
    FResult::err((), e)
}

fn ok<T, E>(x: T) -> FResult<(), T, E> {
    FResult::ok((), x)
}

fn parse_color_string(s: &str) -> FResult<(), Color, Error> {
    match s {
        "black" => ok(Color::Black),
        "red" => ok(Color::Red),
        "green" => ok(Color::Green),
        "yellow" => ok(Color::Yellow),
        "blue" => ok(Color::Blue),
        "magenta" => ok(Color::Magenta),
        "purple" => ok(Color::Magenta),
        "cyan" => ok(Color::Cyan),
        "white" => ok(Color::White),
        "bright black" => ok(Color::BrightBlack),
        "bright red" => ok(Color::BrightRed),
        "bright green" => ok(Color::BrightGreen),
        "bright yellow" => ok(Color::BrightYellow),
        "bright blue" => ok(Color::BrightBlue),
        "bright magenta" => ok(Color::BrightMagenta),
        "bright cyan" => ok(Color::BrightCyan),
        "bright white" => ok(Color::BrightWhite),
        _ => {
            let msg = Head::new().text("Unknown color").quoted(s);
            let err = Error::make(msg, ERROR_PARSE_COLOR);
            error(err)
        }
    }
}

fn parse_color_num(yml: &Yaml) -> FResult<(), u8, Error> {
    match yml {
        Yaml::Integer(i) if *i >= 0 && *i <= 255 => ok(*i as u8),
        Yaml::Integer(i) => {
            let msg = Head::new()
                .text("Invalid color number")
                .quoted(&i.to_string());
            let err = Error::make(msg, ERROR_PARSE_COLOR)
                .add_hint(Hint::new().text("Color number must be in range 0..255"));
            error(err)
        }
        _ => {
            let msg = Head::new().text("Invalid color number");
            let err = Error::make(msg, ERROR_PARSE_COLOR)
                .add_hint(Hint::new().text("Color number must be an integer"));
            error(err)
        }
    }
}

fn parse_color(yml: &Yaml) -> FResult<(), Color, Errors> {
    match &yml {
        Yaml::String(s) => parse_color_string(s).into_errors(),
        Yaml::Array(a) if a.len() != 3 => {
            let msg = Head::new().text("Invalid color format");
            let err = Error::make(msg, ERROR_PARSE_COLOR)
                .add_hint(Hint::new().text("Color must be an array of 3 integers"));
            error(err).into_errors()
        }
        Yaml::Array(a) => FResult::make((), Ok(()))
            .combine(|()| parse_color_num(a.first().unwrap()), |(), r| r)
            .combine(|()| parse_color_num(a.get(1).unwrap()), |r, g| (r, g))
            .combine(
                |()| parse_color_num(a.get(2).unwrap()),
                |(r, g), b| Color::TrueColor { r, g, b },
            ),
        _ => {
            let msg = Head::new().text("Invalid color");
            let err = Error::make(msg, ERROR_PARSE_COLOR)
                .add_hint(Hint::new().text("Color must be a string or an array of 3 integers"));
            error(err).into_errors()
        }
    }
}

fn parse_style_str(s: &str) -> FResult<(), Styles, Error> {
    match s {
        "clear" => ok(Styles::Clear),
        "bold" => ok(Styles::Bold),
        "dimmed" => ok(Styles::Dimmed),
        "italic" => ok(Styles::Italic),
        "underline" => ok(Styles::Underline),
        "blink" => ok(Styles::Blink),
        "hidden" => ok(Styles::Hidden),
        "strikethrough" => ok(Styles::Strikethrough),
        _ => {
            let msg = Head::new().text("Unknown style");
            let err = Error::make(msg, ERROR_PARSE_STYLE)
                .add_hint(Hint::new().text("Style must be one of"))
                .add_hint(Hint::new().text("-").important("clear"))
                .add_hint(Hint::new().text("-").important("bold"))
                .add_hint(Hint::new().text("-").important("dimmed"))
                .add_hint(Hint::new().text("-").important("italic"))
                .add_hint(Hint::new().text("-").important("underline"))
                .add_hint(Hint::new().text("-").important("blink"))
                .add_hint(Hint::new().text("-").important("hidden"))
                .add_hint(Hint::new().text("-").important("strikethrough"));
            error(err)
        }
    }
}

fn parse_style(yml: &Yaml) -> FResult<(), Styles, Errors> {
    match yml {
        Yaml::String(s) => parse_style_str(s).into_errors(),
        _ => {
            let msg = Head::new().text("Invalid style");
            let err = Error::make(msg, ERROR_PARSE_STYLE)
                .add_hint(Hint::new().text("Style must be a string"));
            error(err).into_errors()
        }
    }
}

fn parse_many_style(yml: &Yaml) -> FResult<(), Vec<Styles>, Errors> {
    match yml {
        Yaml::Array(a) => a.iter().fold(ok(Vec::new()), |f_res, yml_style| {
            f_res.combines(
                |()| parse_style(yml_style),
                |mut styles, style| {
                    styles.push(style);
                    styles
                },
            )
        }),
        _ => {
            let msg = Head::new().text("Invalid style");
            let err = Error::make(msg, ERROR_PARSE_STYLE)
                .add_hint(Hint::new().text("Style must be an array of strings"));
            error(err).into_errors()
        }
    }
}

fn parse_color_info_key(key: &str, val: &Yaml) -> FResult<(), ColorInfoKey, Errors> {
    match key {
        "fg" => parse_color(val).map_res(ColorInfoKey::Fg),
        "bg" => parse_color(val).map_res(ColorInfoKey::Bg),
        "styles" => parse_many_style(val).map_res(ColorInfoKey::Styles),
        _ => {
            let msg = Head::new().text("Unknown key").quoted(key);
            let err = Error::make(msg, ERROR_PARSE_COLOR)
                .add_hint(Hint::new().text("Key must be one of"))
                .add_hint(Hint::new().text("-").important("fg"))
                .add_hint(Hint::new().text("-").important("bg"))
                .add_hint(Hint::new().text("-").important("styles"));
            error(err).into_errors()
        }
    }
}

fn parse_color_info(yml: &Yaml) -> FResult<(), ColorInfo, Errors> {
    match yml {
        Yaml::Hash(map) => map.iter().fold(ok(ColorInfo::default()), |f_res, (k, v)| {
            f_res.combines(
                |()| match k {
                    Yaml::String(k) => parse_color_info_key(k, v),
                    _ => {
                        let msg = Head::new().text("Invalid key");
                        let err = Error::make(msg, ERROR_PARSE_COLOR)
                            .add_hint(Hint::new().text("Key must be a string"));
                        error(err).into_errors()
                    }
                },
                |color_info, color_info_key| match color_info_key {
                    ColorInfoKey::Fg(c) => color_info.fg_color(c),
                    ColorInfoKey::Bg(c) => color_info.bg_color(c),
                    ColorInfoKey::Styles(styles) => color_info.styles(styles),
                },
            )
        }),
        _ => {
            let msg = Head::new().text("Invalid color info");
            let err = Error::make(msg, ERROR_PARSE_COLOR)
                .add_hint(Hint::new().text("Color info must be a map"));
            error(err).into_errors()
        }
    }
}

fn parse_width(yml: &Yaml) -> FResult<(), usize, Error> {
    match yml {
        Yaml::Integer(i) if *i >= 0 => ok(*i as usize),
        _ => {
            let msg = Head::new().text("Invalid width");
            let err = Error::make(msg, ERROR_PARSE_THEME_FILE)
                .add_hint(Hint::new().text("Width must be a positive integer"));
            error(err)
        }
    }
}

fn parse_theme_key(key: &str, value: &Yaml) -> FResult<(), ThemeKey, Errors> {
    match key {
        "width" => parse_width(value).into_errors().map_res(ThemeKey::Width),
        "keyword" => parse_color_info(value).map_res(ThemeKey::Keyword),
        "operator" => parse_color_info(value).map_res(ThemeKey::Operator),
        "def_var" => parse_color_info(value).map_res(ThemeKey::DefVar),
        "expr_var" => parse_color_info(value).map_res(ThemeKey::ExprVar),
        "number" => parse_color_info(value).map_res(ThemeKey::Number),
        "ty_var" => parse_color_info(value).map_res(ThemeKey::TyVar),
        _ => {
            let msg = Head::new().text("Unknown key").quoted(key);
            let err = Error::make(msg, ERROR_PARSE_THEME_FILE)
                .add_hint(Hint::new().text("Key must be one of"))
                .add_hint(Hint::new().text("-").important("width"))
                .add_hint(Hint::new().text("-").important("keyword"))
                .add_hint(Hint::new().text("-").important("operator"))
                .add_hint(Hint::new().text("-").important("def_var"))
                .add_hint(Hint::new().text("-").important("expr_var"))
                .add_hint(Hint::new().text("-").important("number"))
                .add_hint(Hint::new().text("-").important("ty_var"));
            error(err).into_errors()
        }
    }
}

fn parse_theme_map(map: &yaml::Hash) -> FResult<(), Vec<ThemeKey>, Errors> {
    map.iter().fold(ok(Vec::new()), |f_res, (k, v)| {
        f_res.combines(
            |()| match k {
                Yaml::String(k) => parse_theme_key(k, v),
                _ => {
                    let msg = Head::new().text("Invalid key");
                    let err = Error::make(msg, ERROR_PARSE_THEME_FILE)
                        .add_hint(Hint::new().text("Key must be a string"));
                    error(err).into_errors()
                }
            },
            |mut vec, theme_key| {
                vec.push(theme_key);
                vec
            },
        )
    })
}

fn parse_theme(yml: &Yaml) -> FResult<(), Vec<ThemeKey>, Errors> {
    match yml {
        Yaml::Hash(map) => parse_theme_map(map),
        _ => {
            let msg = Head::new().text("Invalid theme");
            let err = Error::make(msg, ERROR_PARSE_THEME_FILE)
                .add_hint(Hint::new().text("Theme must be a map"));
            error(err).into_errors()
        }
    }
}

fn parse_yml_vec(yml: &[Yaml]) -> Result<Vec<ThemeKey>, Errors> {
    if yml.len() != 1 {
        let msg = Head::new().text("Invalid theme file");
        let err = Error::make(msg, ERROR_PARSE_THEME_FILE)
            .add_hint(Hint::new().text("Theme file must contain only one document"));
        Err(Errors::from(err))
    } else {
        parse_theme(yml.first().unwrap()).get_res()
    }
}

/// parse [ThemeKey] from yml file
pub fn parse_yml_keys(file_name: &str, content: String) -> Result<Vec<ThemeKey>, Errors> {
    parse_yml(file_name, content).and_then(|yml_vec| parse_yml_vec(&yml_vec))
}
