//! OpenAPI documentation routes.
//!
//! This module defines routes related to serving OpenAPI documentation for the
//! `gedeair_backend` application. It provides endpoints to access the OpenAPI
//! schema and possibly other related documentation, allowing clients to understand
//! and interact with the API.
//!
//! # Features
//! - Serving the OpenAPI schema in JSON format.
//! - Providing documentation for the API endpoints as specified by the OpenAPI standard.

use utoipa::OpenApi;

use crate::models::file::FileType;
use crate::models::profile::user::User;
use crate::routes::user::me::__path_get_me;
use crate::routes::utils::download::__path_download_file;
use crate::routes::utils::login::__path_get_login;
use crate::routes::utils::logout::__path_get_logout;
use crate::routes::utils::status::__path_get_status;
use crate::routes::utils::upload::{FileSchema, __path_post_upload_files};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_status,
        get_login,
        get_logout,
        get_me,
        post_upload_files,
        download_file,
    ),
    components(schemas(User), schemas(FileType), schemas(FileSchema),)
)]
struct ApiDoc;

/// Configures the OpenAPI documentation routes.
///
/// This function sets up the routes for serving OpenAPI documentation.
/// It configures the Swagger UI to be available at: `/swagger-ui`
pub fn openapi(path: &str) -> axum::Router<crate::state::AppState> {
    let path = match path {
        "/" => "",
        _ => path,
    };

    axum::Router::new().merge(
        utoipa_swagger_ui::SwaggerUi::new(format!("{path}/swagger-ui"))
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
}
