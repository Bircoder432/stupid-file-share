use crate::storage::SharedStorage;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

const ADMIN_TOKEN: &str = "supersecretadmintoken";

pub async fn admin_delete_handler(
    Path(short): Path<String>,
    headers: axum::http::HeaderMap,
    State(storage): State<SharedStorage>,
) -> impl IntoResponse {
    match headers.get("authorization") {
        Some(value) => {
            let token = value.to_str().unwrap_or_default();
            if token != format!("Bearer {}", ADMIN_TOKEN) {
                return (StatusCode::UNAUTHORIZED, "Invalid token");
            }
        }
        None => return (StatusCode::UNAUTHORIZED, "Missing token"),
    }

    let mut lock = storage.lock().await;
    if lock.uploads.remove(&short).is_some() {
        (StatusCode::OK, "Deleted")
    } else {
        (StatusCode::NOT_FOUND, "Short not found")
    }
}
