use crate::store::object_store::read_object;

pub fn execute(hash: &str) -> std::io::Result<()> {
    let content_bytes = read_object(hash)?;
    let content = String::from_utf8_lossy(&content_bytes);

    println!("\x1b[32m--- Content Start ---\x1b[0m");
    println!("{}", content);
    println!("\x1b[32m--- Content End ---\x1b[0m");
    Ok(())
}
