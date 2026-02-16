use axum::{Json, Router, routing::post};

use crate::swap::{SwapError, SwapRequest, build_swap_psbt};

/// Build the API router with all routes.
pub fn router() -> Router {
    Router::new().route("/swap", post(create_swap))
}

/// POST /swap — Build a swap PSBT from a SwapRequest.
async fn create_swap(Json(request): Json<SwapRequest>) -> Result<Json<SwapResponse>, ApiError> {
    let psbt = build_swap_psbt(&request).map_err(ApiError::Swap)?;
    Ok(Json(SwapResponse {
        psbt_base64: psbt.to_string(),
    }))
}

/// Response body for the swap endpoint.
#[derive(serde::Serialize)]
pub struct SwapResponse {
    /// The unsigned PSBT encoded as a base64 string.
    pub psbt_base64: String,
}

/// API error wrapper.
pub enum ApiError {
    Swap(SwapError),
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::Swap(e) => (axum::http::StatusCode::BAD_REQUEST, e.to_string()),
        };
        let body = serde_json::json!({ "error": message });
        (status, Json(body)).into_response()
    }
}

/// Start the API server, binding to the given address.
pub async fn serve(bind: &str) {
    let app = router();
    let listener = tokio::net::TcpListener::bind(bind)
        .await
        .expect("failed to bind API server");
    println!("API server listening on {bind}");
    axum::serve(listener, app).await.expect("API server error");
}
