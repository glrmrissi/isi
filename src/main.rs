use sha1::{Sha1, Digest};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use flate2::write::ZlibEncoder;
use flate2::Compression;

fn hash_and_store_blob(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let header = format!("blob {}\0", content.len());
    
    let mut full_data = header.into_bytes();
    full_data.extend(content);

    let mut hasher = Sha1::new();
    hasher.update(&full_data);
    let hash = format!("{:x}", hasher.finalize());

    save_to_objects(&hash, &full_data)?;

    Ok(hash)
}

fn save_to_objects(hash: &str, data: &[u8]) -> io::Result<()> {
    let (dir, file_name) = hash.split_at(2);
    let path = format!(".lixo/objects/{}", dir);
    fs::create_dir_all(&path)?;

    let full_path = format!("{}/{}", path, file_name);
    let file = File::create(full_path)?;

    let mut encoder = ZlibEncoder::new(file, Compression::default());
    encoder.write_all(data)?;
    encoder.finish()?;

    Ok(())
}

fn main() -> io::Result<()> {
    fs::create_dir_all(".isi/objects")?;

    let hash = hash_and_store_blob("foo.txt")?;
    println!("Hash: {}", hash);
    println!("Verify in: .isi/objects/{}/{}", &hash[0..2], &hash[2..]);
    
    Ok(())
}