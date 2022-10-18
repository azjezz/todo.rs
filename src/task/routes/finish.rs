use crate::http::RedirectResponse;
use crate::task::database::repository;

use actix_web::web;

pub async fn route(
    repository: web::Data<repository::TaskRepository>,
    id: web::Path<(i32,)>,
) -> RedirectResponse {
    repository.finish(id.into_inner().0);

    RedirectResponse::to("/".to_string())
}
