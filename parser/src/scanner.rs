use location::Span;

pub trait Mark: Copy {
    /// get marker
    fn position(&self) -> usize;
}

pub trait Scanner {
    type Mark: Mark;

    /// Returns the next character without consuming it.
    fn peek(&self) -> Option<char>;

    /// Consumes and returns the next character.
    fn next(&mut self) -> Option<char>;

    /// Consumes and returns the next character if it satisfies the predicate.
    fn consume_if(&mut self, predicate: impl Fn(char) -> bool) -> Option<char> {
        if let Some(c) = self.peek()
            && predicate(c)
        {
            self.next()
        } else {
            None
        }
    }

    /// Returns true if input is fully consumed.
    fn is_eof(&self) -> bool {
        self.peek().is_none()
    }

    /// Creates a checkpoint for backtracking.
    fn checkpoint(&self) -> Self::Mark;

    /// Restores the scanner to a previous checkpoint.
    fn rollback(&mut self, mark: Self::Mark);

    /// Creates a span from a checkpoint to the current position.
    fn span_from(&self, start: Self::Mark) -> Span;
}

// ===========================================================================
// Basic Marker
// ===========================================================================
#[derive(Debug, Clone, Copy)]
pub struct BasicMarker(usize);

impl Mark for BasicMarker {
    fn position(&self) -> usize {
        self.0
    }
}

impl From<usize> for BasicMarker {
    fn from(position: usize) -> Self {
        BasicMarker(position)
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
    type Mark = BasicMarker;

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos..)?.chars().next()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn checkpoint(&self) -> Self::Mark {
        Self::Mark::from(self.pos)
    }

    fn rollback(&mut self, mark: Self::Mark) {
        self.pos = mark.position();
    }

    fn span_from(&self, start: Self::Mark) -> Span {
        Span::new(start.position(), self.pos)
    }
}
