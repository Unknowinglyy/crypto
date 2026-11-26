use anyhow::{Ok, Result};
use std::{fs, path::Path};

use crate::set1::c3::DecryptGuess;
use crate::set1::c3::single_byte_xor;

// TODO: challenge says:
// "One of the 60-character strings in this file has been encrypted by single-character XOR.
// Find it."
// should chunks be used when the file does not perfectly divide into 60-character strings?
// something else with chunks be better like chunks with a remainder?
// const CHUNK_SIZE: usize = 60;

// Goes through the provided file of CHUNK_SIZE'd strings and provides its best guess of the
// plaintext that is created from decrypting each chunk via single character XOR.
pub fn detect_single_char_xor<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = fs::read(path)?;
    let lines = file
        .split(|&b| b == b'\n')
        .map(|line| line.strip_prefix(b"\r").unwrap_or(line));

    let mut score: f64 = f64::INFINITY;
    let mut guess = DecryptGuess::new(vec![]);

    for line in lines {
        let res = single_byte_xor(line)?;
        //didn't find a valid guess within the entire line
        if res.chi_score.is_none() {
            continue;
        }

        let curr_score = res.chi_score.unwrap();
        if curr_score < score {
            score = curr_score;
            guess = res;
        }
    }

    Ok(guess.plaintext.unwrap())
}

#[cfg(test)]
mod tests {
    use super::detect_single_char_xor;
    use std::path::PathBuf;

    #[test]
    fn test_detect_single_char_xor() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("data/4.txt");
        let res = detect_single_char_xor(path).unwrap();
        assert_eq!(res, "TRVTH NVKE");
    }
}
