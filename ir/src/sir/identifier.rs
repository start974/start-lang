use crate::sir::{Documentation, Variable};
use location::{Location, SourceId, Span, Spanned};
use std::collections::HashMap;
use std::fmt;

// ============================================================================
// Definition Identifier
// ============================================================================

/// identifier of a definition (expression or type), unique in its file;
/// only a `DefTable` creates one, in source order, so a redefined name gets a
/// new one
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefId(usize);

impl DefId {
    /// index of the definition in its file, in source order
    pub fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for DefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

// ============================================================================
// Definition Information
// ============================================================================

/// what a definition defines
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefKind {
    Expression,
    Type,
}

/// what is known about a definition before typing; its file is the one of
/// its `DefTable`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefInfo {
    /// defined variable: name and span of the name
    pub var: Variable,
    pub kind: DefKind,
    /// span of the whole definition (`a : Nat := 3`, `T := Nat`)
    pub span: Span,
    pub doc: Option<Documentation>,
}

impl DefInfo {
    /// name of the definition
    pub fn name(&self) -> &str {
        self.var.name()
    }
}

impl Spanned for DefInfo {
    fn span(&self) -> Span {
        self.span
    }
}

// ============================================================================
// Definition Table
// ============================================================================

/// every definition of a file, indexed by its `DefId`, with an index by
/// name; only the definitions get an identifier, the uses keep their name
/// until name resolution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefTable {
    /// file of the definitions
    source: SourceId,
    /// indexed by `DefId`, in source order
    defs: Vec<DefInfo>,
    /// name → its definitions, in source order
    by_name: HashMap<String, Vec<DefId>>,
}

impl Default for DefTable {
    fn default() -> Self {
        Self::new(SourceId::Unknown)
    }
}

impl DefTable {
    /// empty table of the definitions of `source`
    pub fn new(source: SourceId) -> Self {
        Self {
            source,
            defs: Vec::new(),
            by_name: HashMap::new(),
        }
    }

    /// file of the definitions
    pub fn source(&self) -> &SourceId {
        &self.source
    }

    /// give a new identifier to the definition of `var`, whose whole
    /// definition spans `span`
    pub fn define(
        &mut self,
        var: Variable,
        kind: DefKind,
        span: Span,
        doc: Option<Documentation>,
    ) -> DefId {
        let id = DefId(self.defs.len());
        self.by_name.entry(var.value.clone()).or_default().push(id);
        self.defs.push(DefInfo {
            var,
            kind,
            span,
            doc,
        });
        id
    }

    /// information on the definition `id`
    ///
    /// # Panics
    /// if `id` comes from another table
    pub fn get(&self, id: DefId) -> &DefInfo {
        &self.defs[id.0]
    }

    /// name of the definition `id`
    pub fn name(&self, id: DefId) -> &str {
        self.get(id).name()
    }

    /// where the definition `id` is: its file and the span of the whole
    /// definition
    pub fn location(&self, id: DefId) -> Location<'_> {
        Location::new(&self.source, self.get(id).span)
    }

    /// where the name of the definition `id` is
    pub fn name_location(&self, id: DefId) -> Location<'_> {
        Location::new(&self.source, self.get(id).var.span())
    }

    /// documentation of the definition `id`
    pub fn doc(&self, id: DefId) -> Option<&Documentation> {
        self.get(id).doc.as_ref()
    }

    /// last definition named `name`
    pub fn last(&self, name: &str) -> Option<DefId> {
        self.all(name).last().copied()
    }

    /// every definition named `name`, in source order
    pub fn all(&self, name: &str) -> &[DefId] {
        self.by_name.get(name).map_or(&[], Vec::as_slice)
    }

    /// every definition, in source order
    pub fn iter(&self) -> impl Iterator<Item = (DefId, &DefInfo)> {
        self.defs
            .iter()
            .enumerate()
            .map(|(i, info)| (DefId(i), info))
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn var(name: &str, start: usize) -> Variable {
        Variable::new(name.to_string(), Span::new(start, start + name.len()))
    }

    fn doc(line: &str) -> Documentation {
        Documentation::from(vec![line.to_string()])
    }

    #[test]
    fn def_table_gives_one_id_per_definition() {
        let mut defs = DefTable::default();
        assert!(defs.is_empty());
        let a0 = defs.define(
            var("a", 0),
            DefKind::Expression,
            Span::new(0, 6),
            Some(doc("first a")),
        );
        let t = defs.define(
            var("T", 10),
            DefKind::Type,
            Span::new(10, 18),
            Some(doc("type T")),
        );
        let a1 = defs.define(var("a", 20), DefKind::Expression, Span::new(20, 26), None);

        assert_ne!(a0, a1);
        assert_eq!(defs.len(), 3);
        assert_eq!(defs.name(a0), "a");
        assert_eq!(defs.get(t).kind, DefKind::Type);
        assert_eq!(defs.get(a1).var.span(), Span::new(20, 21));
        assert_eq!(defs.doc(a0), Some(&doc("first a")));
        assert_eq!(defs.doc(a1), None);
        let ids: Vec<_> = defs.iter().map(|(id, _)| id.index()).collect();
        assert_eq!(ids, [0, 1, 2]);
    }

    #[test]
    fn def_table_by_name() {
        let mut defs = DefTable::default();
        let a0 = defs.define(var("a", 0), DefKind::Expression, Span::new(0, 6), None);
        defs.define(var("b", 5), DefKind::Expression, Span::new(5, 11), None);
        let a1 = defs.define(var("a", 10), DefKind::Expression, Span::new(10, 16), None);

        assert_eq!(defs.all("a"), [a0, a1]);
        assert_eq!(defs.last("a"), Some(a1));
        assert_eq!(defs.all("c"), []);
        assert_eq!(defs.last("c"), None);
    }

    #[test]
    fn def_table_locations() {
        // `T := Nat.` then `a : T := 3.` in `test.st`
        let source = SourceId::File(PathBuf::from("test.st"));
        let mut defs = DefTable::new(source.clone());
        defs.define(var("T", 0), DefKind::Type, Span::new(0, 8), None);
        let a = defs.define(var("a", 10), DefKind::Expression, Span::new(10, 20), None);

        assert_eq!(defs.source(), &source);
        assert_eq!(defs.get(a).span(), Span::new(10, 20));
        assert_eq!(defs.location(a), Location::new(&source, Span::new(10, 20)));
        assert_eq!(
            defs.name_location(a),
            Location::new(&source, Span::new(10, 11))
        );
    }
}
