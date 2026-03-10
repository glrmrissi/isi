use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use sha1::{Digest, Sha1};

use crate::store::object_store::save_to_objects;

pub fn create_and_store_commit(tree_hash: &str, parent: Option<&str>, message: &str) -> io::Result<String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut body = format!("tree {tree_hash}\n");
    if let Some(p) = parent {
        body.push_str(&format!("parent {p}\n"));
    }
    body.push_str(&format!("author isi <isi@local> {timestamp} +0000\n"));
    body.push_str(&format!("committer isi <isi@local> {timestamp} +0000\n"));
    body.push('\n');
    body.push_str(message);
    body.push('\n');

    let header = format!("commit {}\0", body.len());
    let mut full_data = header.into_bytes();
    full_data.extend(body.as_bytes());

    let mut hasher = Sha1::new();
    hasher.update(&full_data);
    let hash = format!("{:x}", hasher.finalize());

    save_to_objects(&hash, &full_data)?;
    Ok(hash)
}
