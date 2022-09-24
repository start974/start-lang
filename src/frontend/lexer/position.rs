#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position{
    pub line: usize,
    offset: usize,
}

impl Position {
    pub fn new(line: usize, offset: usize) -> Position {
        assert!(line > 0 && offset > 0);
        Position{line: line, offset: offset}
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn incr(&mut self) {
        self.offset += 1;
    }

    // pub fn decr(&mut self) {
    //     assert!(self.offset > 0);
    //     self.offset -= 1;
    // }

    pub fn pred(&self) -> Self {
        Self { offset: self.offset - 1, line: self.line }
    }
    pub fn next_line(&mut self) {
        self.line += 1;
        self.offset = 1;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { line:1, offset: 1 }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FilePosition {
     path: String,
     start: Position,
     end: Position,
}

impl FilePosition{
    pub fn new(path: &str, start: Position, end: Position) -> FilePosition{
        assert!(start.line <= end.line, "start line > end line");
        assert!(start.line != end.line || start.offset <= end.offset, "start offset > end offset on same line");
        FilePosition{path: path.to_string(), start: start, end: end}
    }

    // pub fn path(&self) -> &String {
    //     &self.path
    // }

    pub fn start(&self) -> &Position {
        &self.start
    }

    pub fn end(&self) -> &Position {
        &self.end
    }
}
