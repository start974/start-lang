use crate::meta_info::{GetMetaInfo, MetaInfo, PrettyMetaInfo, SetMetaInfo};
use crate::{AsCharacter, AsIdentifier, AsNumber};
use location::{GetSpan, Span};
use pp::pretty::*;

#[derive(Debug, Clone, PartialEq, Eq)]
//#[deprecated(note = "Use MetaTrait")]
pub struct Meta<T> {
    meta: MetaInfo,
    pub value: T,
    span: Span,
}

impl<T> Meta<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self {
            meta: MetaInfo::default(),
            value,
            span,
        }
    }

    /// map value
    pub fn map<U, F>(self, f: F) -> Meta<U>
    where
        F: FnOnce(T) -> U,
    {
        Meta {
            value: f(self.value),
            meta: self.meta,
            span: self.span,
        }
    }
}

impl<T> GetMetaInfo for Meta<T> {
    fn meta_info(&self) -> &MetaInfo {
        &self.meta
    }
}

impl<T> SetMetaInfo for Meta<T> {
    fn set_meta_info(&mut self, meta: MetaInfo) {
        self.meta = meta;
    }
}

impl<T> std::fmt::Display for Meta<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> GetSpan for Meta<T> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> PrettyMetaInfo for Meta<T>
where
    T: Pretty,
{
    fn pretty_inner(&self, theme: &Theme) -> Doc<'_> {
        self.value.pretty(theme)
    }
}

impl<T> Pretty for Meta<T>
where
    T: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        <Self as PrettyMetaInfo>::pretty(self, theme)
    }
}

impl<T> AsIdentifier for Meta<T>
where
    T: AsIdentifier,
{
    fn name(&self) -> &str {
        self.value.name()
    }
}

impl<T> AsNumber for Meta<T>
where
    T: AsNumber,
{
    fn as_number(&self) -> &num_bigint::BigUint {
        self.value.as_number()
    }
}

impl<T> AsCharacter for Meta<T>
where
    T: AsCharacter,
{
    fn as_character(&self) -> char {
        self.value.as_character()
    }
}
