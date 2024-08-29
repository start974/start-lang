use super::super::location::Location;

pub struct Error {
    expect: String,
    location: Location,
    content: Vec<String>,
}

impl Error {
    pub fn new(expect: &str, lines: &[String], location: &Location) -> Self {
        Error {
            expect: expect.to_string(),
            location: location.clone(),
            content: location.content(lines),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error: expected {}.", self.expect)?;
        writeln!(f, "--> {}", self.location)?;
        let digits = std::cmp::max(
            (self.location.pos_end.row + 1).to_string().len(),
            (self.location.pos_start.row + 1).to_string().len(),
        );
        // TODO size of file
        let mut i = self.location.pos_start.row;
        for line in &self.content {
            writeln!(f, "{:width$} | {line}", i, width = digits, line = line)?;
            i += 1;
        }

        if self.location.pos_start.row == self.location.pos_end.row {
            let start = self.location.pos_start.column;
            let end = self.location.pos_end.column;
            let indicator = " ".repeat(start + digits + 3) + &"^".repeat(end - start);
            writeln!(f, "{indicator:}")?;
        };
        Ok(())
    }
}

pub struct Errors {
    errors: Vec<Error>,
}

impl Errors {
    fn new() -> Self {
        Errors { errors: Vec::new() }
    }

    pub fn error(expect: &str, lines: &[String], location: &Location) -> Self {
        let error = Error::new(expect, lines, location);
        let mut errors = Errors::new();
        errors.errors.push(error);
        errors
    }

    pub fn concat(mut self, errors: Errors) -> Self {
        self.errors.extend(errors.errors);
        self
    }

    //pub fn is_empty(&self) -> bool {
    //self.errors.is_empty()
    //}
}

impl std::fmt::Debug for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            writeln!(f, "{error:?}")?;
        }
        Ok(())
    }
}

pub type ErrorsResult<T> = Result<T, Errors>;
