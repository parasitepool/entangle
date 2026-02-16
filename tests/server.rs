use axum::body::Body;
use axum::http::{Request, StatusCode};
use leptos::config::get_configuration;
use tower::ServiceExt;

use crate::*;

#[tokio::test]
async fn with_api_nesting() {
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let app = entangle::server::router(leptos_options, true);
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
