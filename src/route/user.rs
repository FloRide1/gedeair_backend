use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{artifact::UserResponse, oidc::User};

#[utoipa::path(
        get,
        path = "/user",
        responses(
            (status = 200, description = "The User is logged in and return his detail information"),
            (status = 404, description = "The User is not logged in")
        )
    )]
pub async fn me(
    user: Option<User>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, impl IntoResponse)> {
    match user {
        Some(user) => Ok((StatusCode::OK, Json(user.into()))),
        None => Err((StatusCode::NOT_FOUND, "You're not logged in")),
    }
}
