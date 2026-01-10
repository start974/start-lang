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
