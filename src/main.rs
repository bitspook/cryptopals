mod text_utils;
mod set1;

fn main() -> anyhow::Result<()> {
    let cypher = set1::str_encrypt_repeating_key_xor("ICE", "HELLO WORLD");

    println!("CYPHER: {}", cypher);

    Ok(())
}
