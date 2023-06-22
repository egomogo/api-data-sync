#[derive(Debug)]
pub enum Error {
    Unexpected
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unexpected => write!(f, "Unexpected Error")
        }
    }
}