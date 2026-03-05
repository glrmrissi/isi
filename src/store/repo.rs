use std::env;
use std::io;
use std::path::PathBuf;

pub fn find_root() -> io::Result<PathBuf> {
    let mut current = env::current_dir()?;
    loop {
        if current.join(".isi").is_dir() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Not an isi repository (no .isi directory found)",
            ));
        }
    }
}
