use errors::{Error, Errors};

// ============================================================================
// Partial
// ============================================================================

/// result of a pass: what it could build, even with errors, and every error
/// it found (each one says how the pass corrected it to go on)
#[must_use]
#[derive(Debug, Default)]
pub struct Partial<T> {
    pub value: T,
    pub errors: Errors,
}

impl Partial<()> {
    /// no value yet, no error: the start of a value built from several
    /// partial ones (`and_then`, `zip`, `collect`)
    pub fn new() -> Self {
        Partial {
            value: (),
            errors: Errors::default(),
        }
    }
}

impl<T> Partial<T> {
    /// a value at hand, without error
    pub fn ok(value: T) -> Self {
        Partial::new().map(|()| value)
    }

    /// no error
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// add an error
    pub fn with_error(mut self, error: Error) -> Self {
        self.errors.extend([error]);
        self
    }

    /// add errors
    pub fn with_errors(mut self, errors: impl IntoIterator<Item = Error>) -> Self {
        self.errors.extend(errors);
        self
    }

    /// change the value, keep the errors
    pub fn map<U, F>(self, f: F) -> Partial<U>
    where
        F: FnOnce(T) -> U,
    {
        Partial {
            value: f(self.value),
            errors: self.errors,
        }
    }

    /// go on with the value, even if it has errors: the errors of both
    pub fn and_then<U, F>(self, f: F) -> Partial<U>
    where
        F: FnOnce(T) -> Partial<U>,
    {
        f(self.value).with_errors(self.errors)
    }

    /// both values, and the errors of both
    pub fn zip<U>(self, other: Partial<U>) -> Partial<(T, U)> {
        Partial {
            value: (self.value, other.value),
            errors: self.errors,
        }
        .with_errors(other.errors)
    }

    /// the value if there is no error, else the errors
    pub fn into_result(self) -> Result<T, Errors> {
        self.into()
    }
}

impl<T> From<Partial<T>> for Result<T, Errors> {
    fn from(partial: Partial<T>) -> Self {
        if partial.is_ok() {
            Ok(partial.value)
        } else {
            Err(partial.errors)
        }
    }
}

impl<T> From<Result<T, Errors>> for Partial<T>
where
    T: Default,
{
    fn from(result: Result<T, Errors>) -> Self {
        match result {
            Ok(value) => Partial::ok(value),
            Err(errors) => Partial::ok(T::default()).with_errors(errors),
        }
    }
}

/// collect the values, and the errors of all
impl<T, C> FromIterator<Partial<T>> for Partial<C>
where
    C: FromIterator<T>,
{
    fn from_iter<I: IntoIterator<Item = Partial<T>>>(iter: I) -> Self {
        let mut errors = Errors::default();
        let value = iter
            .into_iter()
            .map(|partial| {
                errors.extend(partial.errors);
                partial.value
            })
            .collect();
        Partial { value, errors }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use errors::Message;

    fn error(code: i32) -> Error {
        Error::new(code, Message::text("error"))
    }

    fn codes<T>(partial: &Partial<T>) -> Vec<i32> {
        (&partial.errors).into_iter().map(Error::code).collect()
    }

    #[test]
    fn value_kept_with_errors() {
        let partial = Partial::ok(vec![1, 2]).with_error(error(1));
        assert!(!partial.is_ok());
        assert_eq!(partial.value, [1, 2]);
        assert_eq!(partial.into_result().err().map(|errs| errs.len()), Some(1));
    }

    #[test]
    fn ok_without_error() {
        let partial = Partial::ok("value");
        assert!(partial.is_ok());
        assert_eq!(partial.into_result().ok(), Some("value"));
        assert!(Partial::new().is_ok());
    }

    #[test]
    fn map_keeps_errors() {
        let partial = Partial::ok(2).with_error(error(1)).map(|n| n * 10);
        assert_eq!(partial.value, 20);
        assert_eq!(codes(&partial), [1]);
    }

    #[test]
    fn and_then_goes_on_and_keeps_all_errors() {
        let partial = Partial::ok(2)
            .with_error(error(1))
            .and_then(|n| Partial::ok(n + 1).with_error(error(2)));
        assert_eq!(partial.value, 3);
        assert_eq!(codes(&partial), [2, 1]);
    }

    #[test]
    fn zip_both() {
        let partial = Partial::ok('a')
            .with_error(error(1))
            .zip(Partial::ok(1).with_errors([error(2), error(3)]));
        assert_eq!(partial.value, ('a', 1));
        assert_eq!(codes(&partial), [1, 2, 3]);
    }

    #[test]
    fn collect_values_and_errors() {
        let partial: Partial<Vec<_>> = (1..=3)
            .map(|n| {
                let partial = Partial::ok(n);
                if n % 2 == 0 {
                    partial.with_error(error(n))
                } else {
                    partial
                }
            })
            .collect();
        assert_eq!(partial.value, [1, 2, 3]);
        assert_eq!(codes(&partial), [2]);
    }

    #[test]
    fn from_result() {
        let ok: Partial<Vec<i32>> = Ok(vec![1]).into();
        assert!(ok.is_ok());
        let err: Partial<Vec<i32>> = Err(Errors::from(error(4))).into();
        assert!(err.value.is_empty());
        assert_eq!(codes(&err), [4]);
    }
}
