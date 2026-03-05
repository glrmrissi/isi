use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::objects::blob::hash_and_store_blob;
use crate::store::ignore::IsiIgnore;
use crate::store::index::add_to_index;
use crate::store::repo::find_root;

pub fn execute(path: &str) -> io::Result<()> {
    let repo_root = find_root()?;
    let ignore = IsiIgnore::load();
    let p = Path::new(path);

    if p.is_dir() {
        add_dir(p, &repo_root, &ignore)
    } else {
        let abs = fs::canonicalize(p)?;
        let rel = relative_to(&abs, &repo_root)?;
        if ignore.should_ignore(&rel, false) {
            println!("ignored: {rel}");
            return Ok(());
        }
        add_file(&abs, &rel)
    }
}

fn add_file(fs_path: &Path, repo_path: &str) -> io::Result<()> {
    match hash_and_store_blob(fs_path.to_str().unwrap()) {
        Ok(hash) => {
            add_to_index(&hash, repo_path)?;
            println!("{hash}  {repo_path}");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error adding file {}: {e}", fs_path.display());
            Err(e)
        }
    }
}

fn add_dir(dir: &Path, repo_root: &PathBuf, ignore: &IsiIgnore) -> io::Result<()> {
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
            add_dir(&path, repo_root, ignore)?;
        } else {
            add_file(&abs, &rel)?;
        }
    }
    Ok(())
}

fn relative_to(abs: &Path, root: &PathBuf) -> io::Result<String> {
    abs.strip_prefix(root)
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Path is outside the repository"))
}