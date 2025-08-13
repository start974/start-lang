use tower_lsp::lsp_types::Position;

#[derive(Debug)]
pub struct PositionMemo {
    content: String,
    lines_offset: Vec<usize>,
}

impl PositionMemo {
    /// make a new bufferis
    pub fn new(content: String) -> Self {
        Self {
            content,
            lines_offset: vec![0],
        }
    }

    /// get position with binary search
    fn get(&mut self, offset: usize) -> Position {
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

    /// compute position wich is after the last line computed
    fn compute(&mut self, offset: usize) -> Position {
        assert!(
            offset <= self.content.len(),
            "Offset out of bounds (offset : {offset} > {}",
            self.content.len()
        );

        let last_offset = self.lines_offset.last().cloned().unwrap_or(0) + 1;
        let mut line = self.lines_offset.len() - 1;
        let mut col = 0;
        for (k, chr) in self.content[last_offset..].chars().enumerate() {
            let current_offset = last_offset + k;
            if offset >= current_offset {
                col += 1;
            }
            if chr == '\n' {
                self.lines_offset.push(current_offset + 1);
                if offset <= current_offset {
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
        let last_offset = self.lines_offset.last().unwrap();
        if &offset == last_offset {
            Position {
                line: (self.lines_offset.len() - 1) as u32,
                character: 0,
            }
        } else if &offset < last_offset {
            self.get(offset)
        } else {
            self.compute(offset)
        }
    }

    /// get content
    pub fn content(&self) -> &str {
        &self.content
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
        let mut memo = PositionMemo::new("Hello\nWorld\nThis is a \ntest".to_string());
        assert_eq!(
            memo.position(0),
            Position {
                line: 0,
                character: 0
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
            memo.position(5),
            Position {
                line: 0,
                character: 5
            }
        );
        assert_eq!(
            memo.position(6),
            Position {
                line: 1,
                character: 0
            }
        );
        assert_eq!(
            memo.position(11),
            Position {
                line: 1,
                character: 5
            }
        );

        assert_eq!(
            memo.position(23),
            Position {
                line: 3,
                character: 1
            }
        );
        assert_eq!(
            memo.position(12),
            Position {
                line: 2,
                character: 0
            }
        );
    }
}
