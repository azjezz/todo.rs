use std::fmt::Display;

use actix_web::http::StatusCode;
use serde_json::{json, Value};

pub enum Error {
    NotFound(String),
    DatabaseConnection(String),
    DatabaseQuery(String),
}

impl Error {
    pub fn get_template(&self) -> String {
        match self {
            Error::NotFound(_) => "errors/404.html".to_owned(),
            _ => "errors/500.html".to_owned(),
        }
    }

    pub fn get_context(&self) -> Value {
        json!({ "message": self.to_string() })
    }

    pub fn get_status(&self) -> StatusCode {
        match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::DatabaseConnection(ref inner) => write!(f, "DatabaseConnection: {}", inner),
            Error::DatabaseQuery(ref inner) => write!(f, "DatabaseQuery: {}", inner),
            Error::NotFound(ref inner) => write!(f, "NotFound: {}", inner),
        }
    }
}
