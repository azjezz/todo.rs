use actix_web::{
    body::BoxBody, http::header::ContentType, web::Data, HttpRequest, HttpResponse, Responder,
};
use serde_json::Value;
use tera::{Context, Tera};

pub struct HtmlTemplateResponse {
    template: String,
    context: Value,
}

impl HtmlTemplateResponse {
    pub fn new(template: &'static str, context: Value) -> Self {
        HtmlTemplateResponse {
            template: template.to_string(),
            context: context,
        }
    }
}

impl Responder for HtmlTemplateResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let tera = req.app_data::<Data<Tera>>().unwrap();

        let content = tera
            .render(&self.template, &Context::from_value(self.context).unwrap())
            .unwrap();

        HttpResponse::Ok()
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
