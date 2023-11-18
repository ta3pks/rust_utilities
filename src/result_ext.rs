pub trait ResultExt<T, E> {
    fn map_into<U>(self) -> Result<U, E>
    where
        U: From<T>;
    fn map_err_into<F>(self) -> Result<T, F>
    where
        F: From<E>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn map_into<U>(self) -> Result<U, E>
    where
        U: From<T>,
    {
        self.map(Into::into)
    }

    fn map_err_into<F>(self) -> Result<T, F>
    where
        F: From<E>,
    {
        self.map_err(Into::into)
    }
}
