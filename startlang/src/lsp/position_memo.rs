use ir::location::Span;
use tower_lsp::lsp_types::{Position, Range};

#[derive(Debug)]
pub struct PositionMemo {
    content: String,
    lines_offset: Vec<usize>,
}

impl PositionMemo {
    /// make a new position memo
    pub fn new(content: String) -> Self {
        Self {
            content,
            lines_offset: vec![0],
        }
    }

    /// get position with binary search
    fn get(&self, offset: usize) -> Position {
        let line = self
            .lines_offset
            .binary_search_by(|&start| start.cmp(&offset))
            .unwrap_or_else(|x| x - 1);

        let col = offset - self.lines_offset[line];
        Position {
            line: line as u32,
            character: col as u32,
        }
    }

    /// compute a position which is after the last line computed
    fn compute(&mut self, offset: usize) -> Position {
        debug_assert!(
            offset <= self.content.len(),
            "Offset out of bounds (offset : {offset} > {}",
            self.content.len()
        );

        let last_offset = *self
            .lines_offset
            .last()
            .expect("the first line starts at 0");
        let mut line = self.lines_offset.len() - 1;
        let mut col = 0;
        for (k, chr) in (last_offset..).zip(self.content[last_offset..].chars()) {
            if offset > k {
                col += 1;
            }
            if chr == '\n' {
                self.lines_offset.push(k + 1);
                if offset <= k {
                    break;
                }
                line += 1;
                col = 0;
            }
        }
        Position {
            line: line as u32,
            character: col as u32,
        }
    }

    pub fn position(&mut self, offset: usize) -> Position {
        let last_offset = *self
            .lines_offset
            .last()
            .expect("the first line starts at 0");
        match offset.cmp(&last_offset) {
            std::cmp::Ordering::Equal => Position {
                line: (self.lines_offset.len() - 1) as u32,
                character: 0,
            },
            std::cmp::Ordering::Less => self.get(offset),
            std::cmp::Ordering::Greater => self.compute(offset),
        }
    }

    /// get range
    pub fn range(&mut self, span: &Span) -> Range {
        let start = self.position(span.start());
        let end = self.position(span.end());
        Range { start, end }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_memo() {
        // line at offset: 7, 12, 13, 24
        let mut memo = PositionMemo::new("123456\n12345\n\n123456789\n123".to_string());
        assert_eq!(
            memo.position(0),
            Position {
                line: 0,
                character: 0
            }
        );
        assert_eq!(
            memo.position(1),
            Position {
                line: 0,
                character: 1
            }
        );
        assert_eq!(
            memo.position(2),
            Position {
                line: 0,
                character: 2
            }
        );
        assert_eq!(
            memo.position(7),
            Position {
                line: 1,
                character: 0
            }
        );
        assert_eq!(
            memo.position(9),
            Position {
                line: 1,
                character: 2
            }
        );
        assert_eq!(
            memo.position(22),
            Position {
                line: 3,
                character: 8
            }
        );
        assert_eq!(
            memo.position(12),
            Position {
                line: 1,
                character: 5
            }
        );
        assert_eq!(
            memo.position(13),
            Position {
                line: 2,
                character: 0
            }
        );
        assert_eq!(
            memo.position(26),
            Position {
                line: 4,
                character: 2
            }
        );
    }
}
