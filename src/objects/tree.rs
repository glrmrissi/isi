use std::io;

use sha1::{Digest, Sha1};
use hex;

use crate::objects::types::TreeEntry;

pub fn create_tree_object(entries: Vec<TreeEntry>) -> io::Result<(String, Vec<u8>)> {
    let mut tree_content = Vec::new();

    for entry in entries {
        let prefix = format!("{} {}\0", entry.mode, entry.name);
        tree_content.extend(prefix.as_bytes());

        let hash_bytes = hex::decode(&entry.hash_hex)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        tree_content.extend(hash_bytes);
    }

    let header = format!("tree {}\0", tree_content.len());
    let mut full_data = header.into_bytes();
    full_data.extend(tree_content);

    let mut hasher = Sha1::new();
    hasher.update(&full_data);
    let tree_hash = format!("{:x}", hasher.finalize());

    Ok((tree_hash, full_data))
}