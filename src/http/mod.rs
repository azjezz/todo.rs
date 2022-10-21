use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Data,
    HttpRequest, HttpResponse, HttpResponseBuilder, Responder as ActixResponder,
};
use tera::{Context, Tera};

use crate::errors::Error;

pub mod response;

pub enum Responder {
    HtmlTemplate(String, Context, StatusCode),
    Error(Error),
    Redirect(String, StatusCode),
}

impl ActixResponder for Responder {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Responder::Redirect(location, status) => HttpResponseBuilder::new(status)
                .append_header(("Location", location))
                .finish(),
            Responder::Error(e) => {
                let tera = match req.app_data::<Data<Tera>>() {
                    Some(t) => t,
                    None => {
                        eprintln!("failed to load tera.");

                        ::std::process::exit(1);
                    }
                };

                let (template, status) = match e {
                    Error::NotFound => ("errors/404.html", StatusCode::NOT_FOUND),
                    _ => ("errors/500.html", StatusCode::INTERNAL_SERVER_ERROR),
                };

                let mut context: Context = Context::new();
                context.insert("message", &e.to_string());

                let content = match tera.render(template, &context) {
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
            Responder::HtmlTemplate(template, context, status) => {
                let tera = match req.app_data::<Data<Tera>>() {
                    Some(t) => t,
                    None => {
                        eprintln!("failed to load tera.");

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
