pub mod admin;
pub mod file;
pub mod list;
pub mod upload;

use crate::storage::SharedStorage;
use axum::{
    Router,
    routing::{delete, get, post},
};

use admin::admin_delete_handler;
use file::file_handler;
use list::list_handler;
use upload::upload_handler;

pub fn routes(storage: SharedStorage) -> Router {
    Router::new()
        .route("/upload", post(upload_handler))
        .route("/{short}/{filename}", get(file_handler))
        .route("/{short}", get(list_handler))
        .route("/admin/{short}", delete(admin_delete_handler)) // DELETE для админа
        .with_state(storage)
}
