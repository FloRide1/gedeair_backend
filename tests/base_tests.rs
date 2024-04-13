mod prepare;

use crate::prepare::prepare_mock_db;
use axum::{body::Body, extract::Request, http::StatusCode};
use gedeair_backend::{app, Arguments};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::util::ServiceExt;

#[tokio::test]
async fn basic_login_test() {
    let mut arguments = Arguments::default();
    arguments.skip_oidc = true;

    let db = prepare_mock_db().await;

    let app = app(db, arguments).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/login")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn basic_me_test() {
    let mut arguments = Arguments::default();
    arguments.skip_oidc = true;

    let db = prepare_mock_db().await;

    let app = app(db, arguments).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/user")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(*body.get("id").unwrap(), json!("1"));
    assert_eq!(*body.get("username").unwrap(), json!("test_username"));
    assert_eq!(*body.get("name").unwrap(), json!("test name"));
    assert_eq!(*body.get("email").unwrap(), json!("test@test.com"));
}
