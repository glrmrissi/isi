use std::fs;
use std::io;

pub fn execute() -> io::Result<()> {
    fs::create_dir_all(".isi/objects")?;
    fs::create_dir_all(".isi/refs")?;
    println!(".isi repository initialized successfully!");
    Ok(())
}