use std::fs;
use std::io;

use crate::store::object_store::read_object_raw;
use crate::store::refs::read_head_ref;
use crate::store::repo::find_root;

pub fn execute(remote: Option<&str>) -> io::Result<()> {
    let base_url = remote
        .map(|s| s.to_string())
        .or_else(|| std::env::var("ISI_REMOTE").ok())
        .unwrap_or_else(|| "http://localhost:3000".to_string());
    let base_url = base_url.trim_end_matches('/');

    let root = find_root()?;

    // Repo name = directory name, overridable via ISI_REPO
    let repo_name = std::env::var("ISI_REPO").unwrap_or_else(|_| {
        root.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("default")
            .to_string()
    });

    let client = reqwest::blocking::Client::new();

    // Ensure repo exists on server
    let _ = client
        .post(format!("{base_url}/repos"))
        .header("content-type", "application/json")
        .body(serde_json::json!({ "name": repo_name }).to_string())
        .send()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Collect all objects
    let objects_dir = root.join(".isi/objects");
    let mut hashes: Vec<String> = Vec::new();
    for prefix_entry in fs::read_dir(&objects_dir)? {
        let prefix_entry = prefix_entry?;
        if !prefix_entry.file_type()?.is_dir() { continue; }
        let prefix = prefix_entry.file_name().into_string().unwrap_or_default();
        if prefix.len() != 2 { continue; }
        for obj_entry in fs::read_dir(prefix_entry.path())? {
            let obj_entry = obj_entry?;
            let rest = obj_entry.file_name().into_string().unwrap_or_default();
            if rest.len() == 38 {
                hashes.push(format!("{prefix}{rest}"));
            }
        }
    }

    let total = hashes.len();
    let mut pushed = 0;
    let mut skipped = 0;

    for hash in &hashes {
        let (kind, raw) = read_object_raw(hash)?;

        let resp = client
            .put(format!("{base_url}/objects/{hash}"))
            .header("x-object-kind", &kind)
            .header("content-type", "application/octet-stream")
            .body(raw)
            .send()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        match resp.status().as_u16() {
            200 | 201 => pushed += 1,
            409 => skipped += 1,
            code => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("server returned {code} for object {hash}"),
                ));
            }
        }
    }

    // Push ref
    let (branch, commit_hash) = read_head_ref()?;

    if let Some(hash) = commit_hash {
        let body = serde_json::json!({ "hash": hash });
        let resp = client
            .put(format!("{base_url}/repos/{repo_name}/refs/{branch}"))
            .header("content-type", "application/json")
            .body(body.to_string())
            .send()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        if !resp.status().is_success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("failed to push ref {branch}: {}", resp.status()),
            ));
        }

        println!("repo: {repo_name}");
        println!("pushed {pushed}/{total} objects ({skipped} already existed)");
        println!("{branch} -> {}", &hash[..7]);
    } else {
        println!("nothing to push (no commits yet)");
    }

    Ok(())
}
