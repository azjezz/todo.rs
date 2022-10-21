#![macro_use]

#[macro_export]
macro_rules! __response_template__ {
    ($t:expr, $($json:tt)+) => {
        $crate::http::Responder::HtmlTemplate(
            $t.to_string(),
            ::tera::Context::from_value(::serde_json::json!($($json)+)).unwrap(),
            ::actix_web::http::StatusCode::OK
        )
    };
    ($t:expr) => {
        $crate::http::Responder::HtmlTemplate(
            $t.to_string(),
            ::tera::Context::new(),
            ::actix_web::http::StatusCode::OK
        )
    };
}

macro_rules! __response_redirect_to__ {
    ($t:expr) => {
        $crate::http::Responder::Redirect($t.to_string(), ::actix_web::http::StatusCode::FOUND)
    };
}

macro_rules! __response_error__ {
    ($e:expr) => {
        $crate::http::Responder::Error(crate::errors::Error::from($e))
    };
}

macro_rules! __response_not_found__ {
    () => {
        $crate::http::Responder::Error(crate::errors::Error::NotFound)
    };
}

pub(crate) use __response_error__ as error;
pub(crate) use __response_not_found__ as not_found;
pub(crate) use __response_redirect_to__ as redirect_to;
pub(crate) use __response_template__ as template;
