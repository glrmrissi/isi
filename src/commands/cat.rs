use crate::store::object_store::read_object_with_type;

pub fn execute(hash: &str) -> std::io::Result<()> {
    let (kind, content) = read_object_with_type(hash)?;

    println!("\x1b[32m--- {kind} {hash} ---\x1b[0m");

    match kind.as_str() {
        "tree" => print_tree(&content),
        _ => println!("{}", String::from_utf8_lossy(&content)),
    }

    Ok(())
}

fn print_tree(data: &[u8]) {
    let mut i = 0;
    while i < data.len() {
        // read "mode name\0"
        let null_pos = match data[i..].iter().position(|&b| b == 0) {
            Some(p) => i + p,
            None => break,
        };

        let entry_str = String::from_utf8_lossy(&data[i..null_pos]);
        let mut parts = entry_str.splitn(2, ' ');
        let mode = parts.next().unwrap_or("");
        let name = parts.next().unwrap_or("");

        i = null_pos + 1;

        // next 20 bytes are the binary hash
        if i + 20 > data.len() {
            break;
        }
        let hash_bytes = &data[i..i + 20];
        let hash_hex: String = hash_bytes.iter().map(|b| format!("{b:02x}")).collect();
        i += 20;

        let kind = if mode == "40000" { "tree" } else { "blob" };
        println!("{mode} {kind} {hash_hex}    {name}");
    }
}
