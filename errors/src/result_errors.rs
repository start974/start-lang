use crate::Errors;

pub trait ResultErrors<T> {
    fn combine<U>(self, other: Result<U, Errors>) -> Result<(T, U), Errors>;
}

impl<T> ResultErrors<T> for Result<T, Errors> {
    fn combine<U>(self, other: Result<U, Errors>) -> Result<(T, U), Errors> {
        match (self, other) {
            (Ok(t), Ok(u)) => Ok((t, u)),
            (Err(e1), Err(e2)) => Err(e1.combine(e2)),
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}

pub trait ResultErrorUnit {
    fn combine(self, other: Result<(), Errors>) -> Result<(), Errors>;
}

impl ResultErrorUnit for Result<(), Errors> {
    fn combine(self, other: Result<(), Errors>) -> Result<(), Errors> {
        match (self, other) {
            (Ok(()), Ok(())) => Ok(()),
            (Err(e1), Err(e2)) => Err(e1.combine(e2)),
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}
