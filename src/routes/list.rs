use crate::{models::FileMeta, storage::SharedStorage};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ListResponse {
    pub short: String,
    pub files: Vec<FileInfo>,
}

#[derive(Serialize)]
struct FileInfo {
    pub filename: String,
    pub size: u64,
}

pub async fn list_handler(
    Path(short): Path<String>,
    State(storage): State<SharedStorage>,
) -> Result<Json<ListResponse>, StatusCode> {
    let files: Vec<FileInfo> = {
        let lock = storage.lock().await;
        let upload = match lock.uploads.get(&short) {
            Some(u) => u,
            None => return Err(StatusCode::NOT_FOUND),
        };

        upload
            .files
            .iter()
            .map(|f| FileInfo {
                filename: f.filename.clone(),
                size: f.size,
            })
            .collect()
    };

    Ok(Json(ListResponse { short, files }))
}
