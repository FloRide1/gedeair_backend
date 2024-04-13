use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;

use crate::__path_get_status;
use crate::route::__path_login;
use crate::route::__path_me;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_status,
        login,
        me
    ),
    tags(
        (name = "gedeair_backend", description = "Todo items management API")
    )
)]
struct ApiDoc;

pub fn openapi() -> axum::Router<AppState> {
    axum::Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
