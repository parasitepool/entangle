use axum::Router;
use leptos::config::LeptosOptions;
use leptos_axum::{LeptosRoutes, generate_route_list};

use crate::app::{App, shell};
use crate::listing::ListingStore;

/// Build the server router with Leptos SSR.
///
/// When `with_api` is true, the API routes are nested under `/api`.
pub fn router(leptos_options: LeptosOptions, with_api: bool, store: ListingStore) -> Router {
    let routes = generate_route_list(App);

    let mut app = Router::new();

    if with_api {
        app = app.nest_service("/api", crate::api::router());
    }

    app.leptos_routes(&leptos_options, routes, {
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    })
    .layer(axum::Extension(store))
    .fallback(leptos_axum::file_and_error_handler::<LeptosOptions, _>(
        shell,
    ))
    .with_state(leptos_options)
}

/// Start the frontend server with Leptos SSR.
///
/// If `with_api` is true, the API routes are nested under `/api`.
pub async fn serve(bind: &str, with_api: bool) {
    let conf = leptos::config::get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let store = ListingStore::load("listings.json");

    let app = router(leptos_options, with_api, store);

    let listener = tokio::net::TcpListener::bind(bind)
        .await
        .expect("failed to bind server");
    println!("Server listening on {bind}");
    if with_api {
        println!("API routes available at /api/*");
    }
    axum::serve(listener, app).await.expect("server error");
}
