pub mod openapi;

use axum::routing::get;
use openapi::openapi;

pub fn app() -> axum::Router {
    axum::Router::new()
        .merge(openapi())
        .route("/status", get(get_status))
}

#[utoipa::path(
        get,
        path = "/status",
        responses(
            (status = 200, description = "API is up and functionnal", body = String)
        )
    )]
pub async fn get_status() -> &'static str {
    "UP"
}
