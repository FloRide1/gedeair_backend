use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::get_status,
    ),
    tags(
        (name = "gedeair_backend", description = "Todo items management API")
    )
)]
struct ApiDoc;

pub fn openapi() -> axum::Router {
    axum::Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
