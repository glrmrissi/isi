use std::fs;
use std::io;

use crate::store::index::read_index;
use crate::store::object_store::read_object;
use crate::store::repo::find_root;

pub fn execute(hash_old: Option<&str>, hash_new: Option<&str>) -> io::Result<()> {
    match (hash_old, hash_new) {
        (Some(old), Some(new)) => diff_objects(old, new),
        _ => diff_working_tree(),
    }
}

fn diff_objects(hash_old: &str, hash_new: &str) -> io::Result<()> {
    let old = read_object(hash_old)?;
    let new = read_object(hash_new)?;

    let old_str = String::from_utf8_lossy(&old);
    let new_str = String::from_utf8_lossy(&new);

    print_diff_header(&format!("{}", &hash_old[..7]), &format!("{}", &hash_new[..7]));
    print_diff(&old_str, &new_str);

    Ok(())
}

fn diff_working_tree() -> io::Result<()> {
    let root = find_root()?;
    let entries = read_index()?;

    if entries.is_empty() {
        println!("nothing tracked — run `isi add` first");
        return Ok(());
    }

    let mut any_diff = false;

    for entry in entries {
        let file_path = root.join(&entry.path);

        let stored_bytes = match read_object(&entry.hash) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("warning: object {} not found for {}", &entry.hash[..7], entry.path);
                continue;
            }
        };
        let stored = String::from_utf8_lossy(&stored_bytes);

        if !file_path.exists() {
            any_diff = true;
            println!("\n\x1b[33mdeleted: {}\x1b[0m", entry.path);
            for line in stored.lines() {
                println!("\x1b[31m- {line}\x1b[0m");
            }
            continue;
        }

        let current = fs::read_to_string(&file_path).unwrap_or_default();

        if current == stored.as_ref() {
            continue;
        }

        any_diff = true;
        print_diff_header(&entry.path, "working tree");
        print_diff(&stored, &current);
    }

    if !any_diff {
        println!("no changes since last `isi add`");
    }

    Ok(())
}

fn print_diff_header(a: &str, b: &str) {
    println!("\n\x1b[1m--- {a}\n+++ {b}\x1b[0m");
}

fn print_diff(old: &str, new: &str) {
    for line in diff::lines(old, new) {
        match line {
            diff::Result::Left(l)    => println!("\x1b[31m- {l}\x1b[0m"),
            diff::Result::Both(l, _) => println!("  {l}"),
            diff::Result::Right(r)   => println!("\x1b[32m+ {r}\x1b[0m"),
        }
    }
}