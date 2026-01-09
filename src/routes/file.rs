use crate::{models::FileMeta, storage::SharedStorage};
use axum::{
    body::{Body, Bytes},
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use futures::StreamExt;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn file_handler(
    Path((short, filename)): Path<(String, String)>,
    State(storage): State<SharedStorage>,
) -> Response {
    let file_meta: FileMeta = {
        let lock = storage.lock().await;
        let upload = match lock.uploads.get(&short) {
            Some(u) => u,
            None => return (StatusCode::NOT_FOUND, "Short not found").into_response(),
        };

        match upload.files.iter().find(|f| f.filename == filename) {
            Some(f) => f.clone(),
            None => return (StatusCode::NOT_FOUND, "File not found").into_response(),
        }
    };

    let file = match File::open(&file_meta.path).await {
        Ok(f) => f,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Open failed").into_response(),
    };

    let stream = ReaderStream::new(file);
    let body_stream = stream.map(|result| {
        match result {
            Ok(chunk) => Ok(Bytes::from(chunk)), // явно конвертим в axum::body::Bytes
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "read error")),
        }
    });
    let body = Body::from_stream(body_stream);

    let mut headers = HeaderMap::new();
    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        )
        .header(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{}\"", file_meta.filename))
                .unwrap(),
        )
        .body(body)
        .unwrap()
}
