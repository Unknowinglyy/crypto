use anyhow::{Result, bail};
/// Takes in two byte slices that are hex encoded and returns a hex encoded Result of the two byte
/// slices XOR'd together
///    TODO: make type of bytes and other_bytes AsRef like so:
///    ...<P: AsRef<Path>>(path: P)...
fn fixed_hex_xor(bytes: &[u8], other_bytes: &[u8]) -> Result<Vec<u8>> {
    if bytes.len() != other_bytes.len() {
        bail!("both inputs into fixed_xor need to have the same length");
    }

    let hex_str = hex::encode(
        hex::decode(bytes)?
            .iter()
            .zip(hex::decode(other_bytes)?.iter())
            .map(|(&c1, &c2)| c1 ^ c2)
            .collect::<Vec<u8>>(),
    );

    Ok(hex_str.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let input1 = b"1c0111001f010100061a024b53535009181c";
        let input2 = b"686974207468652062756c6c277320657965";

        let res = fixed_hex_xor(input1, input2).unwrap();
        assert_eq!(res, b"746865206b696420646f6e277420706c6179");
    }
}
