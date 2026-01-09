use axum::{
    extract::{Multipart, State},
    response::Json,
};
use serde::Serialize;
use std::path::PathBuf;
use tokio::fs;

use crate::{models::FileMeta, shorter::generate_unique_short, storage::SharedStorage};

#[derive(Serialize)]
pub struct UploadResponse {
    pub short: String,
    pub files: Vec<String>,
}

pub async fn upload_handler(
    State(storage): State<SharedStorage>,
    mut multipart: Multipart,
) -> Json<UploadResponse> {
    let short = generate_unique_short(&storage, 6).await;
    let mut file_names = Vec::new();

    let base_dir = PathBuf::from("./stupid-file-share");
    let dir = base_dir.join(&short);
    fs::create_dir_all(&dir).await.unwrap();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(filename) = field.file_name() {
            let filename = filename.to_string();
            let path = dir.join(&filename);

            let data = field.bytes().await.unwrap();
            fs::write(&path, &data).await.unwrap();
            file_names.push(filename.to_string());

            let meta = FileMeta {
                path: path.to_str().unwrap_or_default().to_string(),
                filename: filename,
                size: data.len() as u64,
            };

            storage.lock().await.add_file(&short, meta);
        }
    }
    Json(UploadResponse {
        short,
        files: file_names,
    })
}
