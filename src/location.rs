#[derive(Clone)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    /// make a new position
    pub fn make(row: usize, column: usize) -> Self {
        Position { row, column }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

#[derive(Clone)]
pub struct Location {
    file_name: String,
    lines: Vec<String>,
    start: Position,
    end: Position,
}

impl Location {
    /// make a new location
    pub fn make(file_name: String, lines: &[String], start: Position, end: Position) -> Self {
        Self {
            file_name,
            lines: lines[start.row..end.row + 1].to_vec(),
            start,
            end,
        }
    }

    /// content at location
    pub fn content(&self) -> Vec<String> {
        let mut res = self.lines.clone();
        if res.len() == 1 {
            res[0] = res[0][self.start.column..self.end.column].to_string();
        } else {
            let n = res.len() - 1;
            res[0] = res[0][self.start.column..].to_string();
            res[n] = res[n][..self.end.column].to_string();
        }
        res
    }
    /// text at location
    pub fn text(&self) -> String {
        self.content().join("\n")
    }

    /// number of digits
    fn digits(&self) -> usize {
        std::cmp::max(
            (self.end.row + 1).to_string().len(),
            (self.start.row + 1).to_string().len(),
        )
    }

    /// location
    fn fmt_location(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = self.start.row;
        let end = self.end.row;
        if start == end {
            writeln!(f, "{}:{}", self.file_name, self.start)
        } else {
            writeln!(f, "{}:{}-{}", self.file_name, self.start, self.end)
        }
    }

    /// content
    fn fmt_content(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let digits = self.digits();
        let mut i = self.start.row + 1;
        for line in &self.lines {
            writeln!(f, "{:width$} | {line}", i, width = digits, line = line)?;
            i += 1;
        }
        Ok(())
    }

    /// indicator
    fn fmt_indicator(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.start.row == self.end.row {
            writeln!(
                f,
                "   {:width$}{indicator}",
                " ",
                width = self.start.column + 1,
                indicator = "^".repeat(self.end.column - self.start.column)
            )
        } else {
            Ok(())
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_location(f)?;
        self.fmt_content(f)?;
        self.fmt_indicator(f)
    }
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_location(f)
    }
}

pub trait Located {
    /// location of a node
    fn get_location(&self) -> &Option<Location>;

    /// set optinal location
    fn set_opt_location(self, opt_location: Option<Location>) -> Self;

    /// set location
    fn set_location(self, location: Location) -> Self where Self: Sized {
        self.set_opt_location(Some(location))
    }
}
