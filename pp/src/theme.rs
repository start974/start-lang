use crate::{ColorInfo, Doc, ErrorTheme, MessageTheme};
use colored::Color;
use num_bigint::BigUint;

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

impl Theme {
    /// create a default theme
    pub fn default_theme() -> Self {
        Self {
            width: 80,
            keyword: ColorInfo::default().fg_color(Color::Magenta),
            operator: ColorInfo::default().fg_color(Color::Red),
            def_var: ColorInfo::default().fg_color(Color::Blue).bold(),
            expr_var: ColorInfo::default().fg_color(Color::Blue),
            character: ColorInfo::default().fg_color(Color::Green),
            number: ColorInfo::default().fg_color(Color::Green),
            boolean: ColorInfo::default().fg_color(Color::Green),
            ty_var: ColorInfo::default().fg_color(Color::Yellow).italic(),
            comment: ColorInfo::default().fg_color(Color::BrightBlack).italic(),
            documentation: ColorInfo::default().fg_color(Color::White).italic(),
            error: ErrorTheme {
                head: MessageTheme {
                    width: 120,
                    important: ColorInfo::default().fg_color(Color::Red).bold(),
                    normal: ColorInfo::default().fg_color(Color::Red),
                },
                text: MessageTheme {
                    width: 120,
                    important: ColorInfo::default().fg_color(Color::Red).bold(),
                    normal: ColorInfo::default(),
                },
                note: MessageTheme {
                    width: 120,
                    important: ColorInfo::default().fg_color(Color::Yellow).bold(),
                    normal: ColorInfo::default().fg_color(Color::Yellow),
                },
                label_color: Some(ariadne::Color::Red),
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

    /// pprint type variable
    pub fn ty_var<'a>(&self, var: &impl ToString) -> Doc<'a> {
        Doc::text(var.to_string()).annotate(self.ty_var.clone())
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

    /// pprint comment color
    pub fn comment<'a>(&self, comment: &impl ToString) -> Doc<'a> {
        Doc::text(comment.to_string()).annotate(self.comment.clone())
    }

    pub fn documentation<'a>(&self, doc: &impl ToString) -> Doc<'a> {
        Doc::text(doc.to_string()).annotate(self.documentation.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Pretty;

    #[test]
    fn default_theme() {
        let theme = Theme::default();
        assert_eq!(theme.width, 80);
        assert_eq!(theme.keyword, ColorInfo::default());
        assert_eq!(theme.operator, ColorInfo::default());
        assert_eq!(theme.def_var, ColorInfo::default());
        assert_eq!(theme.expr_var, ColorInfo::default());
        assert_eq!(theme.character, ColorInfo::default());
        assert_eq!(theme.number, ColorInfo::default());
        assert_eq!(theme.boolean, ColorInfo::default());
        assert_eq!(theme.ty_var, ColorInfo::default());
        assert_eq!(theme.comment, ColorInfo::default());
        assert_eq!(theme.documentation, ColorInfo::default());
    }

    #[test]
    fn keyword_theme() {
        struct Keyword;
        impl Pretty for Keyword {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme.keyword(&"keyword")
            }
        }
        let theme = Theme::default();

        assert_eq!(Keyword.make_string(&theme), "keyword");
    }

    #[test]
    fn width_theme() {
        struct LongInput;
        impl Pretty for LongInput {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme
                    .keyword(&"ThisIsAVeryLongKeywordThatShouldTestTheWidthSettingOfTheTheme")
                    .append(Doc::softline())
                    .append(theme.operator(&"+"))
                    .append(Doc::softline())
                    .append(theme.operator(&"a"))
                    .append(Doc::softline())
                    .append(theme.operator(&"+"))
                    .append(Doc::softline())
                    .append(theme.keyword(&"AnotherVeryLongKeywordToTestTheWidth"))
            }
        }
        let theme = Theme::default();
        assert_eq!(
            LongInput.make_string(&theme),
            "ThisIsAVeryLongKeywordThatShouldTestTheWidthSettingOfTheTheme + a +\nAnotherVeryLongKeywordToTestTheWidth"
        );
    }

    #[test]
    fn operator() {
        struct Operator;
        impl Pretty for Operator {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme.operator(&"+")
            }
        }
        let theme = Theme::default();
        assert_eq!(Operator.make_string(&theme), "+");
    }

    #[test]
    fn def_var() {
        struct DefVar;
        impl Pretty for DefVar {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme.def_var(&"x")
            }
        }
        let theme = Theme::default();
        assert_eq!(DefVar.make_string(&theme), "x");
    }

    #[test]
    fn expr_var() {
        struct ExprVar;
        impl Pretty for ExprVar {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme.expr_var(&"y")
            }
        }
        let theme = Theme::default();

        assert_eq!(ExprVar.make_string(&theme), "y");
    }

    #[test]
    fn ty_var() {
        struct TyVar;
        impl Pretty for TyVar {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme.ty_var(&"τ")
            }
        }
        let theme = Theme::default();
        assert_eq!(TyVar.make_string(&theme), "τ");
    }

    #[test]
    fn character() {
        struct Character;
        impl Pretty for Character {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme
                    .character('\\')
                    .append(theme.character('\''))
                    .append(theme.character('\n'))
                    .append(theme.character('\r'))
                    .append(theme.character('\t'))
                    .append(theme.character('a'))
            }
        }
        let theme = Theme::default();
        assert_eq!(
            Character.make_string(&theme),
            "'\\\\''\\\'''\\n''\\r''\\t''a'"
        );
    }

    #[test]
    fn number() {
        struct Number;
        impl Pretty for Number {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme.number(&BigUint::from(1234567890u64))
            }
        }
        let theme = Theme::default();
        assert_eq!(Number.make_string(&theme), "1_234_567_890");
    }

    #[test]
    fn boolean() {
        struct Boolean;
        impl Pretty for Boolean {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme.boolean(true).append(theme.boolean(false))
            }
        }
        let theme = Theme::default();
        assert_eq!(Boolean.make_string(&theme), "truefalse");
    }

    #[test]
    fn comment() {
        struct Comment;
        impl Pretty for Comment {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme.comment(&"(* This is a comment *)")
            }
        }
        let theme = Theme::default();
        assert_eq!(Comment.make_string(&theme), "(* This is a comment *)");
    }

    #[test]
    fn documentation() {
        struct Documentation;
        impl Pretty for Documentation {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                theme.documentation(&"(** This is a doc comment *)")
            }
        }
        let theme = Theme::default();

        assert_eq!(
            Documentation.make_string(&theme),
            "(** This is a doc comment *)"
        );
    }
}
