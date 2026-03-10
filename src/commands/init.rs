use std::fs;
use std::io;

pub fn execute() -> io::Result<()> {
    fs::create_dir_all(".isi/objects")?;
    fs::create_dir_all(".isi/refs/heads")?;
    fs::write(".isi/HEAD", "ref: refs/heads/main\n")?;
    println!(".isi repository initialized successfully!");
    Ok(())
}