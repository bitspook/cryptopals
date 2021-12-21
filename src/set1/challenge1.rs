// [[file:../../readme.org::hex-to-b64][hex-to-b64]]
use anyhow::{Result, Error};

use wasm_bindgen::prelude::*;	

pub fn hex_to_b64(input: &str) -> Result<String> {
    let hex_str = hex::decode(input).map_err(Error::from)?;
    let out = base64::encode(hex_str);

    Ok(out)
}
// hex-to-b64 ends here

// [[file:../../readme.org::*Convert Hex to Base64][Convert Hex to Base64:2]]
#[wasm_bindgen]
pub fn hex_to_b64_web(input: &str) -> String {
    hex_to_b64(input).unwrap()
}
// Convert Hex to Base64:2 ends here

// [[file:../../readme.org::*Convert Hex to Base64][Convert Hex to Base64:3]]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1e1_hex_to_b64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex_to_b64(input).unwrap(), output);
    }
}
// Convert Hex to Base64:3 ends here
