use std::io;

use crate::store::object_store::read_object;
use diff;

pub fn execute(hash_old: &str, hash_new: &str) -> io::Result<()> {
    let old = read_object(hash_old)?;
    let new = read_object(hash_new)?;

    let old_str = String::from_utf8_lossy(&old);
    let new_str = String::from_utf8_lossy(&new);

    println!("\n--- Diff beetween {} and {} ---", &hash_old[..7], &hash_new[..7]);

    for line in diff::lines(&old_str, &new_str) {
        match line {
            diff::Result::Left(l)   => println!("\x1b[31m- {l}\x1b[0m"),
            diff::Result::Both(l, _) => println!("  {l}"),
            diff::Result::Right(r)  => println!("\x1b[32m+ {r}\x1b[0m"),
        }
    }

    Ok(())
}