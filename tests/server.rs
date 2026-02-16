use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::*;

#[tokio::test]
async fn get_index_returns_placeholder() {
    let app = entangle::server::router();
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"entangle frontend - coming soon");
}

#[tokio::test]
async fn with_api_nesting() {
    let app = entangle::server::router().nest("/api", entangle::api::router());
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/swap")
                .header("content-type", "application/json")
                .body(Body::from(swap_request_json(true)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
