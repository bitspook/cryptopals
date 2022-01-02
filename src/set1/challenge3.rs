// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:1]]
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::challenge2::xor;
// Challenge 3: Single-byte XOR cipher:1 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:2]]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_till() {
        let input = "key".as_bytes();
        let expected = "keykeykeykeykeyke".as_bytes();

        assert_eq!(repeat_till(input, 17), expected);
    }
}
// Challenge 3: Single-byte XOR cipher:2 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:3]]
pub fn repeat_till(input: &[u8], length: usize) -> Vec<u8> {
    let mut result = vec![];

    for i in 0..length {
        result.push(input[i % input.len()]);
    }

    result
}
// Challenge 3: Single-byte XOR cipher:3 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:4]]
#[wasm_bindgen]
pub fn fixed_key_xor(hexedCipher: &str, key: u8) -> String {
    let cipher = hex::decode(hexedCipher).unwrap();
    let repeated_key = repeat_till(&[key], cipher.len());
    let result = xor(&cipher, &repeated_key);

    std::str::from_utf8(&result)
        .expect("Invalid utf8 chars in string")
        .to_string()
}
// Challenge 3: Single-byte XOR cipher:4 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:5]]
#[cfg(test)]
mod lf_tests {
    use super::*;

    #[test]
    fn test_letter_frequency() {
        let input = "aaaaccddee";
        let lf = letter_frequency(input);

        assert_eq!(lf.get(&'a'), Some(&0.4));
        assert_eq!(lf.get(&'c'), Some(&0.2));
        assert_eq!(lf.get(&'d'), Some(&0.2));
        assert_eq!(lf.get(&'e'), Some(&0.2));
    }
}
// Challenge 3: Single-byte XOR cipher:5 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:6]]
pub fn letter_frequency(input: &str) -> HashMap<char, f64> {
    let mut lf = HashMap::new();

    for c in input.chars() {
        *lf.entry(c.to_ascii_lowercase()).or_default() += 1.0;
    }

    for v in lf.values_mut() {
        *v /= input.len() as f64;
    }

    lf
}
// Challenge 3: Single-byte XOR cipher:6 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:7]]
#[cfg(test)]
mod lfe_tests {
    use super::letter_frequency_error;

    #[test]
    fn test_letter_frequency_error() {
        let input = "She sells sea shells at the sea shore. Shells are blue and they are white, ocean is blue and it is bright.";
        let error_till_2dec = (letter_frequency_error(input) * 100.0).trunc() / 100.0;

        assert_eq!(error_till_2dec, 0.26);
    }
}
// Challenge 3: Single-byte XOR cipher:7 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:8]]
#[wasm_bindgen]
pub fn letter_frequency_error(input: &str) -> f64 {
    let standard_freq = HashMap::from([
        ('a', 0.08167),
        ('b', 0.01492),
        ('c', 0.20782),
        ('d', 0.04253),
        ('e', 0.12702),
        ('f', 0.02228),
        ('g', 0.02015),
        ('h', 0.06094),
        ('i', 0.06966),
        ('j', 0.00153),
        ('k', 0.00772),
        ('l', 0.04025),
        ('m', 0.02406),
        ('n', 0.06749),
        ('o', 0.07507),
        ('p', 0.01929),
        ('q', 0.00095),
        ('r', 0.05987),
        ('s', 0.06327),
        ('t', 0.09056),
        ('u', 0.02758),
        ('v', 0.00978),
        ('w', 0.02360),
        ('x', 0.00150),
        ('y', 0.01974),
        ('z', 0.00074),
    ]);
    let letter_freq = letter_frequency(input);
    let mut freq_sum: f64 = 0.0;

    for (letter, s_freq) in &standard_freq {
        let freq = letter_freq.get(letter).unwrap_or(&0.0);
        let freq_diff = *freq - *s_freq;
        freq_sum += freq_diff * freq_diff;
    }

    (freq_sum / letter_freq.len() as f64) * 100.0
}
// Challenge 3: Single-byte XOR cipher:8 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:9]]
#[derive(Serialize, Deserialize)]
pub struct Crack {
    key: String,
    plain_text: String,
}

pub fn crack_single_key_xor_cipher(hexedCipher: &str) -> Crack {
    let mut solution: (u8, String, f64) = (0, "".to_string(), 99.0);
    for key in 1..255 {
        let cipher = hex::decode(hexedCipher).unwrap();
        let repeated_key = repeat_till(&[key], cipher.len());
        let result = xor(&cipher, &repeated_key);

        if let Ok(result) = std::str::from_utf8(&result) {
            let lfe = letter_frequency_error(result);

            if lfe < solution.2 {
                solution = (key, result.to_string(), lfe);
            }
        }
    }

    Crack {
        key: solution.0.to_string(),
        plain_text: solution.1,
    }
}
// Challenge 3: Single-byte XOR cipher:9 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:10]]
#[wasm_bindgen]
pub fn crack_single_key_xor_cipher_web(hexedCipher: &str) -> JsValue {
    JsValue::from_serde(&crack_single_key_xor_cipher(hexedCipher)).unwrap()
}
// Challenge 3: Single-byte XOR cipher:10 ends here

// [[file:../../readme.org::*Challenge 3: Single-byte XOR cipher][Challenge 3: Single-byte XOR cipher:11]]
fn has_valid_ascii_chars(sentence: &str) -> bool {
    sentence
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_whitespace())
}
// Challenge 3: Single-byte XOR cipher:11 ends here
