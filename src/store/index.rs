use std::fs;
use std::io;

use super::repo::find_root;

pub struct IndexEntry {
    pub hash: String,
    pub path: String,
}

pub fn read_index() -> io::Result<Vec<IndexEntry>> {
    let root = find_root()?;
    let index_path = root.join(".isi/index");

    if !index_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(index_path)?;
    let entries = content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ' ');
            let hash = parts.next()?.to_string();
            let path = parts.next()?.to_string();
            Some(IndexEntry { hash, path })
        })
        .collect();

    Ok(entries)
}

pub fn write_index(entries: &[IndexEntry]) -> io::Result<()> {
    let root = find_root()?;
    let index_path = root.join(".isi/index");

    let content: String = entries
        .iter()
        .map(|e| format!("{} {}\n", e.hash, e.path))
        .collect();

    fs::write(index_path, content)
}

pub fn add_to_index(hash: &str, path: &str) -> io::Result<()> {
    let mut entries = read_index()?;

    if let Some(entry) = entries.iter_mut().find(|e| e.path == path) {
        entry.hash = hash.to_string();
    } else {
        entries.push(IndexEntry {
            hash: hash.to_string(),
            path: path.to_string(),
        });
    }

    write_index(&entries)
}
