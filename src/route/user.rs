use axum::response::IntoResponse;

use crate::oidc::User;

#[utoipa::path(
        get,
        path = "/user",
        responses(
            (status = 200, description = "The User is logged in and return his detail information"),
            (status = 404, description = "The User is not logged in")
        )
    )]
pub async fn me(_user: Option<User>) -> impl IntoResponse {}
