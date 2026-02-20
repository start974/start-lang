use pp::pretty::*;
#[derive(Debug, Clone)]
pub enum Info {
    /// Comment information, e.g. (* comment *)
    Comment { is_doc: bool, content: Vec<String> },

    /// Information about many lines
    Lines,

    /// Information spaces
    Spaces,
}

impl Info {
    /// true if this info lines
    pub fn is_lines(&self) -> bool {
        matches!(self, Info::Lines)
    }
}

impl Pretty for Info {
    fn pretty(&self, _theme: &Theme) -> Doc<'_> {
        match self {
            Info::Comment { is_doc, content } => {
                let start_str = if *is_doc { "(**" } else { "(*" };
                let start = Doc::text(start_str);
                let nest_length = start_str.len() + 1; // +1 for the space after the start
                let end = Doc::text("*)");

                let mid = Doc::intersperse(content.iter().map(Doc::text), Doc::softline())
                    .group()
                    .nest(nest_length.try_into().unwrap());
                Doc::concat(vec![start, Doc::space(), mid, Doc::space(), end])
            }
            Info::Lines => Doc::hardline().append(Doc::hardline()),
            Info::Spaces => Doc::softline(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Infos {
    infos: Vec<Info>,
}

impl Infos {
    /// append an info to this infos, but if the last info is lines or spaces, do not append
    pub fn append(mut self, info: Info) -> Self {
        match self.infos.last() {
            None => {
                self.infos.push(info);
            }
            Some(info_end) => match (info_end, &info) {
                (Info::Lines, Info::Lines)
                | (Info::Lines, Info::Spaces)
                | (Info::Spaces, Info::Spaces) => (),
                (Info::Spaces, Info::Lines) => {
                    self.infos.pop();
                    self.infos.push(info);
                }
                (_, _) => {
                    self.infos.push(info);
                }
            },
        };
        self
    }

    /// concat informations
    pub fn concat(self, other: Infos) -> Infos {
        other.infos.into_iter().fold(self, Self::append)
    }
}

impl Pretty for Infos {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(self.infos.iter().map(|info| info.pretty(theme)), Doc::nil())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lorem_ipsum() -> Vec<String> {
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse dictum dui nulla, non lobortis mauris posuere sed. Vestibulum vulputate mauris vitae enim euismod euismod.".split(" ").map(String::from).collect()
    }

    #[test]
    fn pretty_comment() {
        let comment = Info::Comment {
            is_doc: false,
            content: lorem_ipsum(),
        };
        let theme = Theme::default();
        let str = comment.make_string(&theme);
        assert_eq!(
            str,
            r"(* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse dictum
   dui nulla, non lobortis mauris posuere sed. Vestibulum vulputate mauris vitae
   enim euismod euismod. *)"
        );
    }

    #[test]
    fn pretty_doc() {
        let comment = Info::Comment {
            is_doc: true,
            content: lorem_ipsum(),
        };
        let theme = Theme::default();
        let str = comment.make_string(&theme);
        assert_eq!(
            str,
            r"(** Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse dictum
    dui nulla, non lobortis mauris posuere sed. Vestibulum vulputate mauris
    vitae enim euismod euismod. *)"
        );
    }

    #[test]
    fn pretty_lines() {
        let info = Info::Lines;
        let theme = Theme::default();
        let str = info.make_string(&theme);
        assert_eq!(str, "\n\n");
    }

    #[test]
    fn append() {
        let mut infos = Infos::default();
        let theme = Theme::default();
        let str = infos.make_string(&theme);
        assert_eq!(str, "");

        infos = infos.append(Info::Spaces);
        let str = infos.make_string(&theme);
        assert_eq!(str, " ");

        infos = infos.append(Info::Lines);
        let str = infos.make_string(&theme);
        assert_eq!(str, "\n\n");

        infos = infos.append(Info::Spaces);
        let str = infos.make_string(&theme);
        assert_eq!(str, "\n\n");

        infos = infos.append(Info::Comment {
            is_doc: false,
            content: vec!["test".into()],
        });
        let str = infos.make_string(&theme);
        assert_eq!(str, "\n\n(* test *)");

        infos = infos.append(Info::Spaces);
        let str = infos.make_string(&theme);
        assert_eq!(str, "\n\n(* test *) ");

        infos = infos.append(Info::Lines);
        let str = infos.make_string(&theme);
        assert_eq!(str, "\n\n(* test *)\n\n");

        infos = infos.append(Info::Spaces);
        let str = infos.make_string(&theme);
        assert_eq!(str, "\n\n(* test *)\n\n");
    }
}
