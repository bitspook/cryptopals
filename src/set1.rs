use anyhow::{bail, Error, Result};
use std::collections::HashMap;

use crate::text_utils::*;

fn hex_to_b64(input: &str) -> Result<String> {
    let hex_str = hex::decode(input).map_err(Error::from)?;
    let out = base64::encode(hex_str);

    Ok(out)
}

/// Takes two equal-length buffers and produces their XOR combination
fn fixed_xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    for i in 0..b1.len() {
        buf.push(b1[i] ^ b2[i]);
    }

    buf
}

fn single_byte_xor_cipher(input: &[u8], key: u8) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for i in input {
        res.push(i ^ key);
    }

    res
}

/// Find the key used to cipher using single-byte xor
fn crack_single_byte_xor_cipher(cipher: &[u8]) -> HashMap<u8, String> {
    let mut possible_results: HashMap<u8, String> = HashMap::new();

    for key in 0..255 {
        let res = single_byte_xor_cipher(cipher, key);

        if let Ok(res) = String::from_utf8(res) {
            if is_plain_sentence(&res) {
                possible_results.insert(key, res);
            }
        }
    }

    possible_results
}

pub fn solve_challenge_4() -> Result<(usize, HashMap<u8, String>)> {
    let input = std::fs::read_to_string("./challenge-4-file.txt")?;
    let input = input.split_ascii_whitespace().into_iter();
    let mut line_num = 0;

    for line in input {
        line_num += 1;

        let decoded_line = hex::decode(line)?;
        let solution = crack_single_byte_xor_cipher(&decoded_line);

        if solution.len() > 0 {
            println!("Found a possible solution for line: {}", line_num);
            println!("Solution: {:#?}", solution);

            return Ok((line_num, solution));
        }
    }

    bail!("No solution");
}

fn encrypt_repeating_key_xor(key: &[u8], input: &[u8]) -> Vec<u8> {
    let mut cypher: Vec<u8> = vec![];
    let mut cursor = 0;

    while cursor < input.len() {
        cypher.push(input[cursor] ^ key[cursor % key.len()]);

        cursor += 1;
    }

    cypher
}

pub fn str_encrypt_repeating_key_xor(key: &str, input: &str) -> String {
    let cypher = encrypt_repeating_key_xor(key.as_bytes(), input.as_bytes());

    hex::encode(cypher)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1e1_hex_to_b64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex_to_b64(input).unwrap(), output);
    }

    #[test]
    fn s1e2_fixed_xor() {
        let b1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let b2 = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let expected = "746865206b696420646f6e277420706c6179";

        assert_eq!(fixed_xor(&b1, &b2), hex::decode(expected).unwrap());
    }

    #[test]
    fn test_single_byte_xor_cipher() {
        let input = "011100101".as_bytes();
        let key: u8 = 1;
        let expected = "100011010".as_bytes();

        assert_eq!(single_byte_xor_cipher(input, key), expected);
    }

    #[test]
    fn s1e3_crack_single_byte_xor_cipher() {
        let input =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        let key = 88;
        let solution = crack_single_byte_xor_cipher(&input);
        let solution_has_key = solution.into_iter().any(|(k, _)| k == key);
        println!("SAY SOMETHING");

        assert_eq!(solution_has_key, true);
    }

    #[test]
    fn s1e4_find_xored_line_in_txt_file() {
        let solution = solve_challenge_4().unwrap();

        assert_eq!(solution.0, 171);
    }

    #[test]
    fn s1e5_ecrypt_repeating_key_xor() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let cypher = encrypt_repeating_key_xor(key.as_bytes(), input.as_bytes());
        let cypher = hex::encode(cypher);

        assert_eq!(cypher, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }
}
