mod artifact;
mod cli;
mod oidc;
mod openapi;
mod route;
mod state;

use std::str::FromStr;

use axum::{error_handling::HandleErrorLayer, response::IntoResponse, routing::get};
use axum_oidc::{error::MiddlewareError, EmptyAdditionalClaims};
pub use cli::Arguments;
use openapi::openapi;
use sea_orm::DatabaseConnection;
use state::AppState;

pub async fn app(database: DatabaseConnection, arguments: Arguments) -> axum::Router {
    let state = AppState {
        pool: database.clone(),
        arguments: arguments.clone(),
    };

    axum::Router::new()
        .merge(openapi())
        .merge(secured_route(&arguments).await)
        .route("/status", get(route::get_status))
        .with_state(state)
}

async fn secured_route(arguments: &Arguments) -> axum::Router<AppState> {
    #[cfg(feature = "test")]
    if arguments.skip_oidc {
        return axum::Router::new()
            .merge(route::required_auth())
            .merge(route::optional_auth());
    }

    let session_layer = oidc::session_layer();

    let oidc_login_service = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(axum_oidc::OidcLoginLayer::<EmptyAdditionalClaims>::new());

    let application_base_url = axum::http::Uri::from_str(&arguments.backend_base_url)
        .expect("Application Base URL should be a valid URL");
    let issuer = arguments.openid_issuer.to_owned();
    let client_id = arguments.openid_client_id.to_owned();
    let client_secret = arguments.openid_client_secret.to_owned();

    let oidc_auth_service = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(
            axum_oidc::OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
                application_base_url,
                issuer,
                client_id,
                client_secret,
                vec![],
            )
            .await
            .unwrap(),
        );

    axum::Router::new()
        .merge(route::required_auth())
        .layer(oidc_login_service)
        .merge(route::optional_auth())
        .layer(oidc_auth_service)
        .layer(session_layer)
}
