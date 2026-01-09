use crate::models::{FileMeta, Upload};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

pub type SharedStorage = Arc<Mutex<Storage>>;

pub struct Storage {
    pub uploads: HashMap<String, Upload>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            uploads: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, short: &str, file: FileMeta) {
        let upload = self
            .uploads
            .entry(short.to_string())
            .or_insert_with(Upload::default);
        upload.files.push(file);
        upload.created_at = Instant::now();
    }

    pub fn get_files(&self, short: &str) -> Option<&Vec<FileMeta>> {
        self.uploads.get(short).map(|u| &u.files)
    }

    pub fn remove(&mut self, short: &str) {
        self.uploads.remove(short);
    }

    pub fn exists(&self, short: &str) -> bool {
        self.uploads.contains_key(short)
    }

    pub fn cleanup_expired(&mut self, ttl: Duration) {
        let expired: Vec<String> = self
            .uploads
            .iter()
            .filter_map(|(short, upload)| {
                if upload.created_at.elapsed() > ttl {
                    Some(short.clone())
                } else {
                    None
                }
            })
            .collect();

        for short in expired {
            if let Some(upload) = self.uploads.remove(&short) {
                for f in upload.files {
                    println!("clean {}", f.filename);
                    let _ = std::fs::remove_dir_all(f.path);
                }
            }
        }
    }
}
