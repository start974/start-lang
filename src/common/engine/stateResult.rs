pub enum StateResult<T, E> {
    End,
    Ok(T),
    Error(E),
}

impl<T, E> Into<StateResult<T, E>> for Result<T, E> {
    fn into(self) -> StateResult<T, E> {
        match self {
            Ok(t) => StateResult::Ok(t),
            Err(e) => StateResult::Error(e),
        }
    }
}

impl<T, E> Into<StateResult<T, E>> for Option<Result<T, E>> {
    fn into(self) -> StateResult<T, E> {
        match self {
            Some(res) => res.into(),
            None => StateResult::End,
        }
    }
}

impl<T, E> Into<Option<Result<T, E>>> for StateResult<T, E> {
    fn into(self) -> Option<Result<T, E>> {
        match self {
            StateResult::End => None,
            StateResult::Ok(t) => Some(Ok(t)),
            StateResult::Error(e) => Some(Err(e)),
        }
    }
}
