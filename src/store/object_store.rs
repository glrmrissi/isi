use std::fs::{self, File};
use std::io::{self, Read, Write};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

pub fn save_to_objects(hash: &str, data: &[u8]) -> io::Result<()> {
    let (dir, file_name) = hash.split_at(2);
    let path = format!(".isi/objects/{dir}");
    fs::create_dir_all(&path)?;
    let full_path = format!("{path}/{file_name}");

    let file = File::create(full_path)?;
    let mut encoder = ZlibEncoder::new(file, Compression::default());
    encoder.write_all(data)?;
    encoder.finish()?;

    Ok(())
}

pub fn read_object(hash: &str) -> io::Result<Vec<u8>> {
    let (dir, file_name) = hash.split_at(2);
    let path = format!(".isi/objects/{dir}/{file_name}");
    let file = File::open(path)?;
    
    let mut decoder = ZlibDecoder::new(file);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded)?;

    let pos = decoded.iter().position(|&b| b == 0)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid Header!!"))?;

    Ok(decoded[(pos + 1)..].to_vec())
}