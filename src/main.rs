mod models;
mod routes;
mod shorter;
mod storage;

use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::storage::{SharedStorage, Storage};
use axum::serve;

#[tokio::main]
async fn main() {
    let storage: SharedStorage = Arc::new(Mutex::new(Storage::new()));

    let storage_clone = storage.clone();
    tokio::spawn(async move {
        let ttl = Duration::from_secs(60 * 60 * 60);
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            println!("Cleaning time!");
            storage_clone.lock().await.cleanup_expired(ttl);
        }
    });

    let app = routes::routes(storage);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    serve(listener, app).await.unwrap();
}
