use std::fs;
use std::io;

use super::repo::find_root;

pub fn read_head_commit() -> io::Result<Option<String>> {
    let root = find_root()?;
    let head_path = root.join(".isi/HEAD");

    if !head_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&head_path)?;
    let content = content.trim();

    if let Some(ref_path) = content.strip_prefix("ref: ") {
        let ref_file = root.join(".isi").join(ref_path);
        if ref_file.exists() {
            let hash = fs::read_to_string(&ref_file)?;
            Ok(Some(hash.trim().to_string()))
        } else {
            Ok(None)
        }
    } else {
        Ok(Some(content.to_string()))
    }
}

pub fn write_head_commit(hash: &str) -> io::Result<()> {
    let root = find_root()?;
    let head_path = root.join(".isi/HEAD");

    let ref_path = if head_path.exists() {
        let content = fs::read_to_string(&head_path)?;
        let trimmed = content.trim().to_string();
        trimmed.strip_prefix("ref: ").map(|r| r.to_string())
    } else {
        None
    };

    if let Some(ref_path) = ref_path {
        let ref_file = root.join(".isi").join(&ref_path);
        if let Some(parent) = ref_file.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(ref_file, format!("{hash}\n"))?;
    } else {
        fs::write(head_path, format!("{hash}\n"))?;
    }

    Ok(())
}
