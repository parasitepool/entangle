use axum::{Router, routing::get};

/// Build the frontend router.
///
/// Currently a placeholder. Will be replaced with Leptos SSR routes.
pub fn router() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> &'static str {
    "entangle frontend - coming soon"
}

/// Start the frontend server.
///
/// If `with_api` is true, the API routes are nested under `/api`.
pub async fn serve(bind: &str, with_api: bool) {
    let mut app = router();

    if with_api {
        app = app.nest("/api", crate::api::router());
    }

    let listener = tokio::net::TcpListener::bind(bind)
        .await
        .expect("failed to bind frontend server");
    println!("Frontend server listening on {bind}");
    if with_api {
        println!("API routes available at /api/*");
    }
    axum::serve(listener, app)
        .await
        .expect("frontend server error");
}
