pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            row: self.row,
            column: self.column,
        }
    }
}

pub struct Location {
    pub file_name: String,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Location {
    /// content of location
    pub fn content(&self, lines: &[String]) -> Vec<String> {
        let i = self.pos_start.row;
        let j = self.pos_end.row;
        let mut res = lines[i..j + 1].to_vec();
        let n = j - i;
        if n == 0 {
            res[0] = res[0][self.pos_start.column..self.pos_end.column].to_string();
        } else {
            res[0] = res[0][self.pos_start.column..].to_string();
            res[n] = res[n][..self.pos_end.column].to_string();
        }
        res
    }

    /// text at location
    pub fn text(&self, lines: &[String]) -> String {
        self.content(lines).join("\n")
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.pos_start.row == self.pos_end.row {
            write!(f, "{}:{}", self.file_name, self.pos_start)
        } else {
            write!(f, "{}:{}-{}", self.file_name, self.pos_start, self.pos_end)
        }
    }
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.file_name, self.pos_start, self.pos_end)
    }
}

pub trait Located {
    /// location of a node
    fn get_location(&self) -> &Option<Location>;

    /// set location
    fn set_location(self, location: Location) -> Self;
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Location {
            file_name: self.file_name.clone(),
            pos_start: self.pos_start.clone(),
            pos_end: self.pos_end.clone(),
        }
    }
}
