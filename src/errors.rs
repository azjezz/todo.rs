#[derive(Debug)]
pub enum Error {
    NotFound,
    DatabaseConnection(r2d2::Error),
    DatabaseQueryResult(diesel::result::Error),
}

impl From<r2d2::Error> for Error {
    fn from(value: r2d2::Error) -> Error {
        Error::DatabaseConnection(value)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Error {
        Error::DatabaseQueryResult(value)
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
