pub mod cli;
pub mod oidc;
pub mod openapi;
pub mod state;

use std::str::FromStr;

use axum::{error_handling::HandleErrorLayer, response::IntoResponse, routing::get};
use axum_oidc::{error::MiddlewareError, EmptyAdditionalClaims};
use cli::Arguments;
use oidc::login;
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
        .route("/status", get(get_status))
        .with_state(state)
}

pub async fn secured_route(arguments: &Arguments) -> axum::Router<AppState> {
    #[cfg(feature = "test")]
    if arguments.skip_oidc {
        return axum::Router::new()
            .merge(required_auth())
            .merge(optional_auth());
    }

    let session_layer = oidc::session_layer();

    let oidc_login_service = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(axum_oidc::OidcLoginLayer::<EmptyAdditionalClaims>::new());

    let application_base_url = axum::http::Uri::from_str(&arguments.application_base_url)
        .expect("Application Base URL should be a valid URL");

    let oidc_auth_service = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(
            axum_oidc::OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
                application_base_url,
                arguments.openid_issuer.to_owned(),
                arguments.openid_client_id.to_owned(),
                Some(arguments.openid_client_secret.to_owned()),
                vec![],
            )
            .await
            .unwrap(),
        );

    axum::Router::new()
        .merge(required_auth())
        .layer(oidc_login_service)
        .merge(optional_auth())
        .layer(oidc_auth_service)
        .layer(session_layer)
}

pub fn required_auth() -> axum::Router<AppState> {
    axum::Router::new().route("/login", get(login))
}

pub fn optional_auth() -> axum::Router<AppState> {
    axum::Router::new()
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
