use axum::{extract::State, response::IntoResponse, routing::get};
pub use user::__path_me;

mod user;

pub fn required_auth() -> axum::Router<crate::state::AppState> {
    axum::Router::new().route("/login", get(login))
}

pub fn optional_auth() -> axum::Router<crate::state::AppState> {
    axum::Router::new().route("/user", get(user::me))
}

#[utoipa::path(
        get,
        path = "/login",
        responses(
            (status = 307, description = "You're not logged in and you should be"),
            (status = 303, description = "You're logged in, now go back to Application_base_url")
        )
    )]
pub async fn login(
    _user: crate::oidc::User,
    State(arguments): State<crate::cli::Arguments>,
) -> impl IntoResponse {
    axum::response::Redirect::to(&arguments.frontend_base_url)
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
