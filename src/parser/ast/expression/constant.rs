use super::super::{Comment, Comments, PrettyWithComments, WithComments};
use crate::utils::format_number;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

#[derive(Debug)]
pub enum Kind {
    Nat(BigUint),
    Char(char),
}

impl Pretty for Kind {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self {
            Kind::Nat(n) => theme.constant(&format_number(n)),
            Kind::Char(c) => theme.constant(&format!("'{}'", c.escape_default())),
        }
    }
}

#[derive(Debug)]
pub struct Constant {
    kind: Kind,
    loc: Location,
    comments: Comments,
}

impl Constant {
    /// make a nat constant
    pub fn nat(v: BigUint, loc: Location) -> Self {
        Self {
            kind: Kind::Nat(v),
            loc,
            comments: Comments::default(),
        }
    }

    /// make a char constant
    pub fn char(c: char, loc: Location) -> Self {
        Self {
            kind: Kind::Char(c),
            loc,
            comments: Comments::default(),
        }
    }

    /// get the kind of the constant
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

impl Located for Constant {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

impl WithComments for Constant {
    fn with_comments_before(mut self, comments: Vec<Comment>) -> Self {
        self.comments = self.comments.with_comments_before(comments);
        self
    }

    fn with_comments_after(mut self, comments: Vec<Comment>) -> Self {
        self.comments = self.comments.with_comments_after(comments);
        self
    }

    fn comments_before(&self) -> &[Comment] {
        self.comments.comments_before()
    }

    fn comments_after(&self) -> &[Comment] {
        self.comments.comments_after()
    }
}

impl PrettyWithComments for Constant {
    type Value = Kind;

    fn value_between_comments(&self) -> &Self::Value {
        self.kind()
    }
}
