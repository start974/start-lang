pub trait ResultExt<T, E> {
    fn combine<U>(self, other: Result<U, Vec<E>>) -> Result<(T, U), Vec<E>>;
}

impl<T, E> ResultExt<T, E> for Result<T, Vec<E>> {
    fn combine<U>(self, other: Result<U, Vec<E>>) -> Result<(T, U), Vec<E>> {
        match (self, other) {
            (Ok(t), Ok(u)) => Ok((t, u)),
            (Err(mut e1), Err(e2)) => {
                e1.extend(e2);
                Err(e1)
            }
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}
