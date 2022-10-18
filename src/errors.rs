use std::fmt::Display;

pub enum Error {
    DatabaseConnectionError(String),
    DatabaseQueryError(String),
    MissingDependencyError(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::DatabaseConnectionError(ref inner) => {
                write!(f, "DatabaseConnectionError: {}", inner)
            }
            Error::DatabaseQueryError(ref inner) => write!(f, "DatabaseQueryError: {}", inner),
            Error::MissingDependencyError(dep) => write!(f, "MissingDependencyError: {}", dep),
        }
    }
}
