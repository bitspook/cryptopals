// [[file:../../readme.org::*Challenge 1: Convert Hex to Base64][Challenge 1: Convert Hex to Base64:1]]
use anyhow::{Result, Error};

use wasm_bindgen::prelude::*;
// Challenge 1: Convert Hex to Base64:1 ends here

// [[file:../../readme.org::*Challenge 1: Convert Hex to Base64][Challenge 1: Convert Hex to Base64:2]]
pub fn hex_to_b64(input: &str) -> Result<String> {
    let hex_str = hex::decode(input).map_err(Error::from)?;
    let out = base64::encode(hex_str);

    Ok(out)
}
// Challenge 1: Convert Hex to Base64:2 ends here

// [[file:../../readme.org::*Challenge 1: Convert Hex to Base64][Challenge 1: Convert Hex to Base64:3]]
#[wasm_bindgen]
pub fn hex_to_b64_web(input: &str) -> String {
    hex_to_b64(input).unwrap()
}
// Challenge 1: Convert Hex to Base64:3 ends here

// [[file:../../readme.org::*Challenge 1: Convert Hex to Base64][Challenge 1: Convert Hex to Base64:4]]
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
// Challenge 1: Convert Hex to Base64:4 ends here
