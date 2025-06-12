use super::ast::*;
use super::error::Error;
use crate::utils::location::{Location, SourceCache, SourceId};
use tree_sitter::Node;

pub struct Parser {
    source_id: SourceId,
    content: String,
    tree: tree_sitter::Tree,
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl Parser {
    /// make new parser
    pub fn new(cache: &SourceCache, source_id: SourceId) -> Self {
        assert!(source_id != SourceId::Unknown, "source id is unknown");
        let content = cache.get(&source_id);
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_start::start_language())
            .unwrap();
        let tree = parser.parse(content, None).expect("Parsing error.");
        Self {
            source_id,
            content: content.to_string(),
            tree,
        }
    }

    /// parse
    pub fn parse(&self) -> Result<Program, Vec<Error>> {
        let node = self.tree.root_node();
        self.program(&node)
    }

    fn loc(&self, node: &Node) -> Location {
        let start = node.start_byte();
        let end = node.end_byte();
        Location::new(start, end, self.source_id.clone())
    }

    fn check<F>(&self, node: &Node, expect: &str, f: F) -> Result<()>
    where
        F: Fn(&str, Location) -> Error,
    {
        if node.grammar_name() != expect {
            Err(f(expect, self.loc(node)))
        } else {
            Ok(())
        }
    }

    fn ident(&self, node: &Node) -> Result<Identifier> {
        self.check(node, "ident", Error::kind)?;
        let loc = self.loc(node);
        let name = node.utf8_text(self.content.as_bytes()).unwrap();
        let ident = Identifier::new(name, loc);
        Ok(ident)
    }

    fn number_n(&self, node: &Node) -> Result<NConst> {
        self.check(node, "number_N", Error::kind)?;
        let n_str = node.utf8_text(self.content.as_bytes()).unwrap();
        let n = if n_str.starts_with("0b") || n_str.starts_with("0B") {
            NConst::parse_bytes(&n_str.as_bytes()[2..], 2)
        } else if n_str.starts_with("0o") || n_str.starts_with("0O") {
            NConst::parse_bytes(&n_str.as_bytes()[2..], 8)
        } else if n_str.starts_with("0x") || n_str.starts_with("0X") {
            NConst::parse_bytes(&n_str.as_bytes()[2..], 16)
        } else {
            NConst::parse_bytes(n_str.as_bytes(), 10)
        }
        .unwrap();
        Ok(n)
    }

    fn constant(&self, node: &Node) -> Result<Constant> {
        let child = node.child(0).unwrap();
        match child.kind() {
            "number_N" => {
                let n = self.number_n(&child)?;
                let loc = self.loc(&child);
                Ok(Constant::n(n, loc))
            }
            _ => Err(Error::kind("constant", self.loc(node))),
        }
    }

    pub fn expression(&self, node: &Node) -> Result<Expression> {
        match node.kind() {
            "constant" => self.constant(node).map(|c| Expression::constant(c)),
            "ident" => {
                let ident = self.ident(node)?;
                if ident.name() == "_" {
                    Err(Error::kind("wildcard", self.loc(node)))
                } else {
                    Ok(Expression::var(ident))
                }
            }
            _ => Err(Error::kind("expression", self.loc(node))),
        }
    }

    fn ty(&self, node: &Node) -> Result<Ty> {
        match node.kind() {
            "ident" => {
                let ident = self.ident(node)?;
                if ident.name() == "_" {
                    Err(Error::kind("wildcard", self.loc(node)))
                } else {
                    Ok(Ty::var(ident))
                }
            }
            _ => Err(Error::kind("type", self.loc(node))),
        }
    }

    fn ty_restr(&self, node: &Node) -> Result<Ty> {
        let mut child = node.child(0).unwrap();
        self.check(&child, ":", Error::operator)?;
        child = child.next_sibling().unwrap();
        self.ty(&child)
    }

    fn expression_definition(&self, node: &Node) -> Result<ExpressionDefinition> {
        //def keyword
        let mut child = node.child(0).unwrap();
        self.check(&child, "def", Error::keyword)?;

        // name of definition
        child = child.next_sibling().unwrap();
        let name = self.ident(&child)?;

        // type restriction
        child = child.next_sibling().unwrap();
        let opt_ty = if child.grammar_name() == "ty_restr" {
            let ty = self.ty_restr(&child)?;
            child = child.next_sibling().unwrap();
            Some(ty)
        } else {
            None
        };

        // def operator
        self.check(&child, ":=", Error::operator)?;

        // body of definition
        child = child.next_sibling().unwrap();
        let body = self.expression(&child)?;

        let mut def = ExpressionDefinition::new(name, body);
        if let Some(ty) = opt_ty {
            def.set_ty(ty)
        }
        Ok(def)
    }

    fn ty_definition(&self, node: &Node) -> Result<TyDefinition> {
        // type keyword
        let mut child = node.child(0).unwrap();
        self.check(&child, "type", Error::keyword)?;

        // identifier
        child = child.next_sibling().unwrap();
        let name = self.ident(&child)?;

        // eq def
        child = child.next_sibling().unwrap();
        self.check(&child, ":=", Error::operator)?;

        // body
        child = child.next_sibling().unwrap();
        let ty = self.ty(&child)?;
        Ok(TyDefinition::new(name, ty))
    }

    fn program_item(&self, node: &Node) -> Result<ProgramItem> {
        match node.child(0).unwrap().kind() {
            "def" => self.expression_definition(node).map(ProgramItem::ExprDef),
            "type" => self.ty_definition(node).map(ProgramItem::TyDef),
            _ => Err(Error::kind("definition", self.loc(node))),
        }
    }

    fn program(&self, node: &Node) -> Result<Program, Vec<Error>> {
        let mut prog = Program::empty();
        let mut errors = self
            .check(node, "program", Error::kind)
            .map(|_| Vec::new())
            .unwrap_or_else(|e| Vec::from([e]));
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            match self.program_item(&child) {
                Ok(item) => prog.add_item(item),
                Err(e) => errors.push(e),
            }
        }
        if errors.is_empty() {
            Ok(prog)
        } else {
            Err(errors)
        }
    }
}
