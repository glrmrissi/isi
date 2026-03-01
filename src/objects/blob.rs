use std::fs::File;
use std::io::{self, Read};
use sha1::{Digest, Sha1};

use crate::store::object_store::save_to_objects;

pub fn hash_and_store_blob(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let header = format!("blob {}\0", content.len());
    let mut full_data = header.into_bytes();
    full_data.extend(content);

    let mut hasher = Sha1::new();
    hasher.update(&full_data);
    let hash = format!("{:x}", hasher.finalize());

    save_to_objects(&hash, &full_data)?;

    Ok(hash)
}