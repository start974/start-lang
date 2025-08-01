use super::{Comment, Comments, PrettyWithComments, WithComments};
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;

pub struct Identifier<Name> {
    name: Name,
    loc: Location,
    comments: Comments,
}

impl<Name> Identifier<Name> {
    /// Creates a new identifier with the given name and location.
    pub fn new(name: Name, loc: Location) -> Self {
        Self {
            name,
            loc,
            comments: Comments::default(),
        }
    }

    /// map name of identifier
    pub fn map_name<F, NewName>(self, f: F) -> Identifier<NewName>
    where
        F: FnOnce(Name) -> NewName,
    {
        Identifier {
            name: f(self.name),
            loc: self.loc,
            comments: self.comments,
        }
    }
}

impl<Name> std::fmt::Display for Identifier<Name>
where
    Name: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<Name> std::fmt::Debug for Identifier<Name>
where
    Name: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Identifier")
            .field("name", &self.name)
            .field("loc", &self.loc)
            .field("comments", &self.comments)
            .finish()
    }
}

impl<Name> Located for Identifier<Name> {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

impl<Name> WithComments for Identifier<Name> {
    fn with_comments_before(mut self, comments: Vec<Comment>) -> Self {
        self.comments.before = comments;
        self
    }

    fn with_comments_after(mut self, comments: Vec<Comment>) -> Self {
        self.comments.after = comments;
        self
    }

    fn comments_before(&self) -> &[Comment] {
        &self.comments.before
    }

    fn comments_after(&self) -> &[Comment] {
        &self.comments.after
    }
}

impl<Name> PrettyWithComments for Identifier<Name>
where
    Name: Pretty,
{
    type Value = Name;

    fn value_between_comments(&self) -> &Self::Value {
        &self.name
    }
}
