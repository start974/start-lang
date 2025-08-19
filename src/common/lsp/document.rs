use crate::typer::ast::Identifier;
use crate::typer::env::IdentifierKind;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkedString, Position, Range};

// ===========================================================================
// Symbol
// ===========================================================================
pub type Symbol = Arc<Identifier>;

// ===========================================================================
// Symbol Range
// ===========================================================================
#[derive(Debug, Clone)]
struct SymbolRange {
    /// start character
    start: u32,
    /// end character
    end: u32,
    /// symbol
    symbol: Symbol,
}
// ===========================================================================
// Symbol Information
// ===========================================================================
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub symbol: Symbol,
    /// documentation
    pub doc: Option<MarkedString>,
    /// kind of identifier
    pub kind: IdentifierKind,
    /// type of indentifier
    pub ty: String,
    /// definition location
    pub def_range: Range,
    /// location of references
    pub refs_range: Vec<Range>,
}

#[derive(Debug, Clone, Default)]
pub struct Document {
    /// symbol information
    symbols: HashMap<Symbol, SymbolInfo>,
    /// range table for quick access `(line -> (char_start, char_end -> symbol)`
    range_table: BTreeMap<u32, Vec<SymbolRange>>,
}

impl Document {
    fn add_symbol_range(&mut self, symbol: Symbol, range: Range) {
        debug_assert_eq!(
            range.start.line, range.end.line,
            "location must be on the same line"
        );
        let line = range.start.line;
        let start = range.start.character;
        let end = range.end.character;
        let range = self.range_table.entry(line).or_default();
        let idx = range
            .binary_search_by(|range| range.start.cmp(&start))
            .unwrap_or_else(|x| x);
        range.insert(idx, SymbolRange { start, end, symbol });
    }

    /// add a symbol information
    pub fn add_symbol(&mut self, symbol_info: SymbolInfo) {
        let symbol = symbol_info.symbol.clone();

        self.add_symbol_range(symbol.clone(), symbol_info.def_range);
        for range in &symbol_info.refs_range {
            self.add_symbol_range(symbol.clone(), *range);
        }

        let old = self.symbols.insert(symbol.clone(), symbol_info);
        debug_assert!(
            old.is_none(),
            "Symbol {} already exists in the document",
            symbol
        );
    }

    fn get_symbol_at_position(&self, position: &Position) -> Option<&SymbolRange> {
        let spans = self.range_table.get(&position.line)?;
        let idx = match spans.binary_search_by(|range| range.start.cmp(&position.character)) {
            Ok(i) => i,            // start == position.charater
            Err(0) => return None, // position.character not in any ranges
            Err(i) => i - 1,       // start < position.character
        };
        let range = &spans[idx];
        if position.character <= range.end {
            Some(range)
        } else {
            None
        }
    }

    /// get hover information on position
    pub fn get_hover(&self, position: &Position) -> Option<Hover> {
        let range = self.get_symbol_at_position(position)?;
        let symbol_info = self.symbols.get(&range.symbol)?;
        let ty_string = {
            let op = match symbol_info.kind {
                IdentifierKind::Expr => ":",
                IdentifierKind::Type => ":=",
            };
            MarkedString::from_language_code(
                "startlang".to_string(),
                format!("{} {} {}", range.symbol.name(), op, symbol_info.ty.clone()),
            )
        };
        let contents = match &symbol_info.doc {
            Some(doc) => {
                let sep = MarkedString::from_markdown("-----".to_string());
                HoverContents::Array(vec![ty_string, sep, doc.clone()])
            }
            None => HoverContents::Scalar(ty_string),
        };
        Some(Hover {
            contents,
            range: Some(Range {
                start: Position {
                    line: position.line,
                    character: range.start,
                },
                end: Position {
                    line: position.line,
                    character: range.end,
                },
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typer::ast::IdentifierBuilder;
    use tower_lsp::lsp_types::{MarkedString, Position, Range};

    #[test]
    fn test_document_hover() {
        let mut doc = Document::default();
        let id_builder = IdentifierBuilder::default();
        let id = id_builder.get("var");
        let symbol = Arc::new(id.as_ref().clone());

        let symbol_info = SymbolInfo {
            symbol: symbol.clone(),
            doc: Some(MarkedString::from_markdown(
                "This is my variable".to_string(),
            )),
            kind: IdentifierKind::Expr,
            ty: "Nat".to_string(),
            def_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 5,
                },
            },
            refs_range: vec![Range {
                start: Position {
                    line: 1,
                    character: 2,
                },
                end: Position {
                    line: 1,
                    character: 7,
                },
            }],
        };
        doc.add_symbol(symbol_info);

        let hover = doc.get_hover(&Position {
            line: 0,
            character: 2,
        });
        let hover = hover.unwrap();
        assert_eq!(
            hover.contents,
            HoverContents::Array(vec![
                MarkedString::from_language_code("startlang".to_string(), "var : Nat".to_string()),
                MarkedString::from_markdown("-----".to_string()),
                MarkedString::from_markdown("This is my variable".to_string()),
            ])
        );
        assert_eq!(
            hover.range,
            Some(Range {
                start: Position {
                    line: 0,
                    character: 0
                },
                end: Position {
                    line: 0,
                    character: 5
                }
            })
        );
    }
}
