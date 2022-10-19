use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Data,
    HttpRequest, HttpResponse, HttpResponseBuilder, Responder as ActixResponder,
};
use serde_json::Value;
use tera::{Context, Tera};

use crate::errors::Error;

pub enum Responder {
    HtmlTemplate(String, Value, StatusCode),
    Redirect(String, StatusCode),
}

impl Responder {
    pub fn html_template(template: &'static str, context: Value) -> Self {
        Responder::HtmlTemplate(template.to_string(), context, StatusCode::OK)
    }

    pub fn error(error: Error) -> Self {
        Responder::HtmlTemplate(
            error.get_template(),
            error.get_context(),
            error.get_status(),
        )
    }

    pub fn redirect_to(location: &'static str) -> Self {
        Responder::Redirect(location.to_string(), StatusCode::FOUND)
    }
}

impl ActixResponder for Responder {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Responder::Redirect(location, status) => HttpResponseBuilder::new(status)
                .append_header(("Location", location))
                .finish(),
            Responder::HtmlTemplate(template, value, status) => {
                let tera = match req.app_data::<Data<Tera>>() {
                    Some(t) => t,
                    None => {
                        eprintln!("failed to load tera.");

                        ::std::process::exit(1);
                    }
                };

                let context = match Context::from_value(value) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("while creating response template context: {}", e);

                        ::std::process::exit(1);
                    }
                };

                let content = match tera.render(&template, &context) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("while rendering response template: {}", e);

                        ::std::process::exit(1);
                    }
                };

                HttpResponseBuilder::new(status)
                    .content_type(ContentType::html())
                    .body(content)
            }
        }
    }
}
