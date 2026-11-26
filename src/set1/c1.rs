use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
pub fn hex_to_b64(bytes: &[u8]) -> Result<String> {
    let hex_str = hex::decode(bytes)?;
    let b64_str = STANDARD.encode(&hex_str);
    Ok(b64_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hex_to_base64() {
        let test = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let b64_str = hex_to_b64(test.as_ref()).unwrap();
        assert_eq!(
            b64_str,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
