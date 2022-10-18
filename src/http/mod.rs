use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Data,
    HttpRequest, HttpResponse, HttpResponseBuilder, Responder,
};
use serde_json::{json, Value};
use tera::{Context, Tera};

use crate::errors::Error;

pub struct HtmlTemplateResponse {
    template: String,
    context: Value,
    status: StatusCode,
}

impl HtmlTemplateResponse {
    pub fn new(template: &'static str, context: Value) -> Self {
        HtmlTemplateResponse {
            template: template.to_string(),
            context: context,
            status: StatusCode::OK,
        }
    }

    pub fn error(error: Error) -> Self {
        HtmlTemplateResponse {
            template: "errors/500.html".to_string(),
            context: json!({ "message": error.to_string() }),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Responder for HtmlTemplateResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let tera = match req.app_data::<Data<Tera>>() {
            Some(t) => t,
            None => {
                eprintln!("failed to load tera.");

                ::std::process::exit(1);
            }
        };

        let context = match Context::from_value(self.context) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("while creating response template context: {}", e);

                ::std::process::exit(1);
            }
        };

        let content = match tera.render(&self.template, &context) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("while rendering response template: {}", e);

                ::std::process::exit(1);
            }
        };

        HttpResponseBuilder::new(self.status)
            .content_type(ContentType::html())
            .body(content)
    }
}

pub struct RedirectResponse {
    location: String,
}

impl RedirectResponse {
    pub fn to(location: &'static str) -> Self {
        RedirectResponse {
            location: location.to_string(),
        }
    }
}

impl Responder for RedirectResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Found()
            .append_header(("Location", self.location))
            .finish()
    }
}
