use rand::{Rng, distr::Alphanumeric};

use crate::storage::SharedStorage;

pub fn generate_short(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub async fn generate_unique_short(storage: &SharedStorage, len: usize) -> String {
    loop {
        let short = generate_short(len);
        if !storage.lock().await.exists(&short) {
            return short;
        }
    }
}
