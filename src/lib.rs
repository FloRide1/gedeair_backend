use axum::routing::get;

pub fn app() -> axum::Router {
    axum::Router::new().route("/status", get(get_status))
}

async fn get_status() -> &'static str {
    "UP"
}
