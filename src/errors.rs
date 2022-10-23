#[derive(Debug)]
pub enum Error {
    NotFound,
    DatabaseConnection(r2d2::Error),
    DatabaseQueryResult(diesel::result::Error),
}

macro_rules! implement_from {
    ($( $kind:ident => $t:ty ),*) => {
        $(impl From<$t> for Error {
            fn from(value: $t) -> Error {
                Error::$kind(value)
            }
        })*
    };
}

implement_from!(
    DatabaseConnection => r2d2::Error,
    DatabaseQueryResult => diesel::result::Error
);

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
