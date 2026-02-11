use std::ops::{Range, RangeInclusive};

use location::{GetSpan, SetSpan, Span};
use pp::pretty::*;

/// unicode character class, e.g. [a-zA-Z0-9_]
#[derive(Debug, Clone)]
pub struct Class {
    /// for [^...] (negated class)
    negated: bool,
    /// express classes as a list of ranges, e.g. [a-zA-Z0-9_] => [('a', 'z'), ('A', 'Z'), ('0', '9'), ('_', '_')]
    ranges: Vec<(char, char)>,

    /// span of the class, e.g. [a-zA-Z0-9_]
    span: Span,
}

impl From<char> for Class {
    fn from(c: char) -> Self {
        Class {
            negated: false,
            ranges: vec![(c, c)],
            span: Span::default(),
        }
    }
}

impl From<Range<char>> for Class {
    fn from(r: Range<char>) -> Self {
        Class {
            negated: false,
            ranges: vec![(r.start, r.end)],
            span: Span::default(),
        }
    }
}

impl From<RangeInclusive<char>> for Class {
    fn from(r: RangeInclusive<char>) -> Self {
        let (start, end) = r.into_inner();
        Class {
            negated: false,
            ranges: vec![(start, end)],
            span: Span::default(),
        }
    }
}

impl Class {
    /// extend classes, e.g. [a-zA-Z] + [0-9] => [a-zA-Z0-9]
    pub fn extend(mut self, other: impl Into<Class>) -> Self {
        let other = other.into();
        assert_eq!(
            self.negated, other.negated,
            "Cannot extend classes with different negation"
        );
        self.ranges.extend(other.ranges.iter());
        self
    }

    /// negate class, e.g. [a-z] => [^a-z]
    pub fn negate(mut self) -> Self {
        self.negated = !self.negated;
        self
    }

    /// mache a character against the class, e.g. [a-zA-Z] matches 'b' but not '1'
    pub fn is_match(&self, c: char) -> bool {
        self.ranges
            .iter()
            .any(|(start, end)| *start <= c && c <= *end)
            ^ self.negated
    }
}

impl GetSpan for Class {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Class {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl Pretty for Class {
    fn pretty(&self, _theme: &pp::pretty::Theme) -> pp::pretty::Doc<'_> {
        let mut doc = pp::pretty::Doc::nil();
        doc = doc.append(Doc::text("["));
        if self.negated {
            doc = doc.append(Doc::text("^"));
        }
        for (start, end) in &self.ranges {
            if start == end {
                doc = doc.append(Doc::text(start.to_string()));
            } else {
                doc = doc
                    .append(Doc::text(start.to_string()))
                    .append(Doc::text("-"))
                    .append(Doc::text(end.to_string()));
            }
        }
        doc.append(Doc::text("]"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just() {
        let class = Class::from('a');
        assert!(class.is_match('a'));
        assert!(!class.is_match('b'));
    }

    #[test]
    fn range() {
        let class = Class::from('a'..='z');
        assert!(class.is_match('a'));
        assert!(class.is_match('m'));
        assert!(class.is_match('z'));
        assert!(!class.is_match('A'));
    }

    #[test]
    fn double_range() {
        let class = Class::from('a'..='z').extend('A'..='Z');
        assert!(class.is_match('a'));
        assert!(class.is_match('m'));
        assert!(class.is_match('z'));
        assert!(class.is_match('A'));
        assert!(class.is_match('M'));
        assert!(class.is_match('Z'));
        assert!(!class.is_match('0'));
    }

    #[test]
    fn negated() {
        let class = Class::from('a'..='z').negate();
        assert!(!class.is_match('a'));
        assert!(class.is_match('A'));
    }

    #[test]
    fn utf8() {
        let class = Class::from('é'..'ü');
        assert!(class.is_match('é'));
        assert!(class.is_match('ö'));
        assert!(class.is_match('ü'));
        assert!(!class.is_match('a'));
    }

    #[test]
    fn greek() {
        let class = Class::from('α'..'ω');
        assert!(class.is_match('α'));
        assert!(class.is_match('μ'));
        assert!(class.is_match('ω'));
        assert!(!class.is_match('a'));
    }

    #[test]
    fn pretty() {
        let class = Class::from('a'..='z').extend('A'..='Z').negate();
        let string = class.make_string(&Theme::default());
        assert_eq!(string, "[^a-zA-Z]");
    }
}
