#[derive(Debug)]
pub enum Error {
    InvalidLatitudeRange,
    InvalidLongitudeRange,
    DbConnectionFailed(sqlx::Error),
    SqlExecutionFailed(sqlx::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
