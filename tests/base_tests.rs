mod prepare;

use crate::prepare::prepare_mock_db;
use axum::{body::Body, extract::Request, http::StatusCode, response::IntoResponse};
use gedeair_backend::{app, Arguments};
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
