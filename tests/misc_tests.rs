mod prepare;

use crate::prepare::prepare_mock_db;
use axum::{body::Body, extract::Request, http::StatusCode};
use gedeair_backend::{app, Arguments};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::util::ServiceExt;

#[tokio::test]
async fn basic_status_test() {
    let mut arguments = Arguments::default();
    arguments.skip_oidc = true;

    let db = prepare_mock_db().await;

    let app = app(db, arguments).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/status")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();

    assert_eq!(body, "UP");
}

#[tokio::test]
async fn basic_swagger_test() {
    let mut arguments = Arguments::default();
    arguments.skip_oidc = true;

    let db = prepare_mock_db().await;

    let app = app(db, arguments).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/swagger-ui")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api-docs/openapi.json")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    // Simple text for beeing sure that openapi is working, don't forget to bump version
    assert_eq!(*body.get("openapi").unwrap(), json!("3.0.3"));
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Arguments::command().debug_assert()
}
