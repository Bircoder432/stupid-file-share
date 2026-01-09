use std::time::Instant;

pub struct Upload {
    pub files: Vec<FileMeta>,
    pub created_at: Instant,
}

impl Default for Upload {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            created_at: Instant::now(),
        }
    }
}

#[derive(Clone)]
pub struct FileMeta {
    pub path: String,
    pub filename: String,
    pub size: u64,
}
