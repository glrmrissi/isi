use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::objects::blob::hash_and_store_blob;
use crate::store::ignore::IsiIgnore;
use crate::store::index::{read_index, write_index, IndexEntry, add_to_index};
use crate::store::repo::find_root;

pub fn execute(paths: &[String]) -> io::Result<()> {
    let repo_root = find_root()?;
    let ignore = IsiIgnore::load();

    for path in paths {
        add_path(path, &repo_root, &ignore)?;
    }

    Ok(())
}

fn add_path(path: &str, repo_root: &PathBuf, ignore: &IsiIgnore) -> io::Result<()> {
    let p = Path::new(path);

    if p.is_dir() {
        let abs_dir = fs::canonicalize(p)?;
        let prefix = relative_to(&abs_dir, repo_root).unwrap_or_default();

        let mut new_entries: Vec<IndexEntry> = Vec::new();
        collect_dir(&abs_dir, repo_root, ignore, &mut new_entries)?;

        let existing = read_index()?;
        let mut merged: Vec<IndexEntry> = existing
            .into_iter()
            .filter(|e| !entry_is_under(&e.path, &prefix))
            .collect();

        for e in &new_entries {
            println!("{}  {}", e.hash, e.path);
        }
        merged.extend(new_entries);
        write_index(&merged)?;
    } else {
        let abs = fs::canonicalize(p)?;
        let rel = relative_to(&abs, repo_root)?;
        if ignore.should_ignore(&rel, false) {
            println!("ignored: {rel}");
            return Ok(());
        }
        let hash = store_file(&abs)?;
        add_to_index(&hash, &rel)?;
        println!("{hash}  {rel}");
    }

    Ok(())
}

fn collect_dir(
    dir: &Path,
    repo_root: &PathBuf,
    ignore: &IsiIgnore,
    out: &mut Vec<IndexEntry>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let is_dir = path.is_dir();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name == ".isi" || name == ".git" {
                continue;
            }
        }

        let abs = fs::canonicalize(&path)?;
        let rel = relative_to(&abs, repo_root)?;

        if ignore.should_ignore(&rel, is_dir) {
            continue;
        }

        if is_dir {
            collect_dir(&abs, repo_root, ignore, out)?;
        } else {
            match store_file(&abs) {
                Ok(hash) => out.push(IndexEntry { hash, path: rel }),
                Err(e) => eprintln!("error adding {}: {e}", abs.display()),
            }
        }
    }
    Ok(())
}

fn store_file(fs_path: &Path) -> io::Result<String> {
    hash_and_store_blob(fs_path.to_str().unwrap())
}

fn entry_is_under(entry_path: &str, prefix: &str) -> bool {
    if prefix.is_empty() {
        return true;
    }
    entry_path == prefix
        || entry_path.starts_with(&format!("{prefix}/"))
}

fn relative_to(abs: &Path, root: &PathBuf) -> io::Result<String> {
    abs.strip_prefix(root)
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Path is outside the repository"))
}