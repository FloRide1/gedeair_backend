use axum::{extract::State, response::IntoResponse, routing::get};

pub fn required_auth() -> axum::Router<crate::state::AppState> {
    axum::Router::new().route("/login", get(login))
}

pub fn optional_auth() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
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
    axum::response::Redirect::to(&arguments.application_base_url)
}
