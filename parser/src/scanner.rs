use location::Span;

pub trait Scanner {
    /// peek the next character without consuming it
    fn peek(&self) -> Option<char>;

    /// consume and return the next character
    fn next(&mut self) -> Option<char>;

    /// make a checkpoint of the current position in the input (for backtracking)
    fn checkpoint(&self) -> usize;

    /// restore the input to a previous position (backtracking)
    fn rollback(&mut self, pos: usize);

    /// make span from a checkpoint to the current position
    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.checkpoint())
    }
}

// ===========================================================================
// String Scanner
// ===========================================================================

/// StringScanner implements Scanner for a String input,
/// using a position index to track the current location in the string
pub struct StringScanner {
    /// input string to be scanned
    input: String,
    /// pointer to the current position in the input string
    pos: usize,
}

impl From<&str> for StringScanner {
    fn from(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
        }
    }
}

impl Scanner for StringScanner {
    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn checkpoint(&self) -> usize {
        self.pos
    }

    fn rollback(&mut self, pos: usize) {
        self.pos = pos;
    }
}
