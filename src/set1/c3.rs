use crate::utils;
use anyhow::{Ok, Result};
use std::println;

const ENGLISH_FREQ: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, // V-Z
];

const ALPHABET_LEN: usize = 26;

pub struct DecryptGuess {
    pub ciphertext: Vec<u8>,
    pub plaintext: Option<String>,
    pub decryption_byte: Option<u8>,
    pub chi_score: Option<f64>,
}

impl DecryptGuess {
    pub fn new(ciphertext: Vec<u8>) -> Self {
        DecryptGuess {
            ciphertext,
            plaintext: None,
            decryption_byte: None,
            chi_score: None,
        }
    }
}

///Takes in a valid UTF-8 text that will then be scored based on how close it resembles english
///using character frequency via a Chi-squared test.
///Returns the score the text got. The lower the score, the more it resembles english.
fn score_text(message: &str) -> f64 {
    //use the lowercase version of the decrypted message as we don't care about the upper vs lower
    //distinction of an alphabetical letter.
    let text = message.to_lowercase();
    let mut score: f64 = 0.0;
    let obs_letters: &mut [usize] = &mut vec![0; ALPHABET_LEN];

    //find the frequency of each letter in the message
    for msg_letter in text.chars() {
        //not counting the frequency of whitespace or non-alphabetic characters
        if msg_letter.is_whitespace() || !msg_letter.is_ascii_alphabetic() {
            continue;
        }

        let idx = (msg_letter as u8 - b'a') as usize;
        obs_letters[idx] += 1;
    }

    //find the total amount of ascii letters in the message
    let total_letters: usize = obs_letters.iter().sum();

    //the expected frequency of any letter 'c' equals the expected freq of 'c' * the total amount of
    //characters that are in the text
    let mut exp_letters: Vec<f64> = Vec::with_capacity(ALPHABET_LEN);
    for freq in ENGLISH_FREQ {
        let res: f64 = freq * total_letters as f64;
        exp_letters.push(res);
    }

    // chi-squared test, where the character 'c' is a variable for any english letter:
    // summation of:
    // (observed number of character 'c' - expected number of character 'c') / expected number of character 'c'
    for (&obs, &exp) in obs_letters.iter().zip(exp_letters.iter()) {
        let obs = obs as f64;

        //avoid DBZ errors
        if exp == 0.0 {
            continue;
        }

        let diff = obs - exp;
        score += diff * diff / exp
    }

    //score should never be perfectly 0 here, so most likely the message had no letters
    //and therefore chi-squared was not calculated.
    if score == 0.0 {
        return f64::INFINITY;
    }

    score
}

/// Takes in a hex encoded byte slice and XORs it with each letter of the alphabet.
///
/// Then computes a score on how closely the resulting XOR'd byte slice resembles english and
/// returns the message that best resembles english (lower score is better) along with the letter
/// that was used to decrypt it.
///
/// If nothing is found, then an empty String along with None will be returned.
pub fn single_byte_xor(bytes: &[u8]) -> Result<DecryptGuess> {
    // smaller the score, the more it resembles english
    let mut score: f64 = f64::INFINITY;
    let mut guess = DecryptGuess::new(bytes.to_vec());

    //all possible values of a byte
    // let alphabet: Vec<u8> = (u8::MIN..=u8::MAX).collect();
    let alphabet: Vec<u8> = (b'A'..=b'Z').collect();
    // let alphabet: Vec<u8> = (b'a'..=b'z').collect();
    for cur_letter in alphabet {
        let bytes =
            hex::decode(bytes).expect("bytes inputted into single_byte_xor should be hex encoded");

        //an array that contains the current letter that will be XOR'd with 'bytes' argument
        let letter_array = vec![cur_letter; bytes.len()];

        let xor_msg = utils::fixed_xor(&bytes, &letter_array)?;
        match str::from_utf8(&xor_msg) {
            Result::Ok(decrypted_msg) => {
                let cur_score = score_text(decrypted_msg);

                println!(
                    "current msg, letter, and score: |{}|, /{}/, _{}_",
                    decrypted_msg, cur_letter as char, cur_score
                );

                //if our score is greater than current sum, then we want to reassign it.
                if cur_score < score {
                    score = cur_score;
                    guess.plaintext = Some(decrypted_msg.to_string());
                    guess.decryption_byte = Some(cur_letter);
                    guess.chi_score = Some(cur_score);
                }
            }

            Err(_) => println!("doesn't produce valid UTF-8 :("),
        }
    }

    Ok(guess)
}

#[cfg(test)]
mod tests {
    use super::single_byte_xor;

    #[test]
    fn test_single_byte_xor_cipher() {
        let test = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let msg = single_byte_xor(test).unwrap();
        assert_eq!(msg.plaintext.unwrap(), "Cooking MC's like a pound of bacon");
        assert_eq!(msg.decryption_byte.unwrap(), b'X');
    }
}
