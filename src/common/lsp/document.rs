use crate::typer::env::IdentifierKind;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkedString, Position, Range};

// ===========================================================================
// Symbol
// ===========================================================================
pub type Symbol = Arc<String>;

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
    /// range table for quick access (line -> char_start, char_end, symbol)
    range_table: BTreeMap<u32, BTreeMap<(u32, u32), Symbol>>,
}

impl Document {
    fn add_symbol_range(&mut self, symbol: Symbol, range: Range) {
        debug_assert_eq!(
            range.start.line, range.end.line,
            "location must be on the same line"
        );
        let line = range.start.line;
        let char_start = range.start.character;
        let char_end = range.end.character;
        self.range_table
            .entry(line)
            .or_default()
            .insert((char_start, char_end), symbol.clone());
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

    fn get_symbol_at_position(&self, position: &Position) -> Option<(Symbol, Range)> {
        use std::ops::Bound;
        let line = position.line;
        let character = position.character;
        let line_map = self.range_table.get(&line)?;
        let bound = Bound::Included((character, character));
        dbg!(bound);
        let mut iter = line_map.range((Bound::Unbounded, Bound::Included(&(character, u32::MAX))));
        let (&(start_char, end_char), symbol) = iter.next_back()?;
        if character >= start_char && character < end_char
        {
            let range = Range {
                start: Position {
                    line,
                    character: start_char,
                },
                end: Position {
                    line,
                    character: end_char,
                },
            };
            Some((symbol.clone(), range))
        } else {
            None
        }
    }

    /// get hover information on position
    pub fn get_hover(&self, position: &Position) -> Option<Hover> {
        let (symbol, range) = self.get_symbol_at_position(position)?;
        let symbol_info = self.symbols.get(&symbol)?;
        let ty_string =
            MarkedString::from_language_code("startlang".to_string(), symbol_info.ty.clone());
        let contents = match &symbol_info.doc {
            Some(doc) => {
                let sep = MarkedString::from_markdown("-----".to_string());
                HoverContents::Array(vec![ty_string, sep, doc.clone()])
            }
            None => HoverContents::Scalar(ty_string),
        };
        Some(Hover {
            contents,
            range: Some(range),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::{MarkedString, Position, Range};

    #[test]
    fn test_document_hover() {
        let mut doc = Document::default();
        let symbol = Arc::new("myVar".to_string());
        let symbol_info = SymbolInfo {
            symbol: symbol.clone(),
            doc: Some(MarkedString::from_markdown(
                "This is my variable".to_string(),
            )),
            kind: IdentifierKind::Expr,
            ty: "Int".to_string(),
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
        dbg!(&doc);

        let hover = doc.get_hover(&Position {
            line: 0,
            character: 2,
        });
        dbg!(&hover);
        let hover = hover.unwrap();
        assert_eq!(
            hover.contents,
            HoverContents::Array(vec![
                MarkedString::from_language_code("startlang".to_string(), "Int".to_string()),
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
