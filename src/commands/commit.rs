use std::io;

use crate::objects::commit::create_and_store_commit;
use crate::objects::tree::create_tree_object;
use crate::objects::types::TreeEntry;
use crate::store::index::read_index;
use crate::store::object_store::save_to_objects;
use crate::store::refs::{read_head_commit, write_head_commit};

pub fn execute(message: &str) -> io::Result<()> {
    let entries = read_index()?;

    if entries.is_empty() {
        println!("nothing to commit (index is empty)");
        return Ok(());
    }

    let tree_entries: Vec<TreeEntry> = entries
        .iter()
        .map(|e| TreeEntry {
            mode: "100644".to_string(),
            name: e.path.clone(),
            hash_hex: e.hash.clone(),
        })
        .collect();

    let (tree_hash, tree_data) = create_tree_object(tree_entries)?;
    save_to_objects(&tree_hash, &tree_data)?;

    let parent = read_head_commit()?;
    let commit_hash = create_and_store_commit(&tree_hash, parent.as_deref(), message)?;

    write_head_commit(&commit_hash)?;

    println!("[{}] {}", &commit_hash[..7], message);
    Ok(())
}
