use anyhow::{Result, bail};
/// Takes in two byte slices and returns a Result of the two byte slices XOR'd together
pub fn fixed_xor(bytes: &[u8], other_bytes: &[u8]) -> Result<Vec<u8>> {
    if bytes.len() != other_bytes.len() {
        bail!("both inputs into fixed_xor need to have the same length");
    }

    let hex_str = bytes
        .iter()
        .zip(other_bytes.iter())
        .map(|(&c1, &c2)| c1 ^ c2)
        .collect::<Vec<u8>>();

    Ok(hex_str)
}
