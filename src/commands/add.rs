use std::io;

use crate::objects::blob::hash_and_store_blob;

pub fn execute(path: &str) -> io::Result<()> {
    match hash_and_store_blob(path) {
        Ok(hash) => {
            println!("Object Saved with Hash:{hash}");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error adding file: {e}");
            Err(e)
        }
    }
}