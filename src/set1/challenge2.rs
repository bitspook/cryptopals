// [[file:../../readme.org::*Challenge 2: Fixed XOR][Challenge 2: Fixed XOR:1]]
use wasm_bindgen::prelude::*;
// Challenge 2: Fixed XOR:1 ends here

// [[file:../../readme.org::*Challenge 2: Fixed XOR][Challenge 2: Fixed XOR:2]]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
	let b1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
	let b2 = hex::decode("686974207468652062756c6c277320657965").unwrap();
	let expected = "746865206b696420646f6e277420706c6179";

	assert_eq!(xor(&b1, &b2), hex::decode(expected).unwrap());
    }

    #[test]
    fn test_hexor() {
	assert_eq!(
	    hexor(
		"1c0111001f010100061a024b53535009181c",
		"686974207468652062756c6c277320657965"
	    ),
	    "746865206b696420646f6e277420706c6179"
	)
    }
}
// Challenge 2: Fixed XOR:2 ends here

// [[file:../../readme.org::*Challenge 2: Fixed XOR][Challenge 2: Fixed XOR:3]]
pub fn xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    for i in 0..b1.len() {
        result.push(b1[i] ^ b2[i]);
    }

    result
}

#[wasm_bindgen]
pub fn hexor(s1: &str, s2: &str) -> String {
    let b1 = hex::decode(s1).unwrap();
    let b2 = hex::decode(s2).unwrap();

    hex::encode(xor(&b1, &b2))
}
// Challenge 2: Fixed XOR:3 ends here
