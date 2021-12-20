use itertools::Itertools;

use super::xor;

/// Hamming distance is the number of differing bits in two strings.
///
/// # Arguments
/// * `s1` - the first string as bytes
/// * `s2` - the second string as bytes
///
/// # Example
///
/// ```rust
/// let s1 = "this is a test";
/// let s1 = s1.as_bytes();
/// let s2 = "wokka wokka!!!";
/// let s2 = s2.as_bytes();
/// assert_eq!(hamming_distance(s1, s2), 37);
/// ```
///
/// # Notes
/// -   Hamming distance b/w two strings of equal length is the number of positions at
///     which the corresponding symbols are different
/// -   It measures minimum number of substitutions required to change one string into
///     the other
///
/// ## Algorithms to find Hamming Distance of two strings
///
/// 1.  Brute force; i.e iterate on binary form of both strings and calculate
///     differing bits
/// 2.  Xor and count 1s; i.e xor the two strings, and count the number of 1s in the
///     result
pub fn hamming_distance(s1: &[u8], s2: &[u8]) -> usize {
    let difference = xor(s1, s2);

    difference
        .into_iter()
        .map(|b| format!("{:08b}", b))
        .join("")
        .chars()
        .filter(|x| x == &'1')

        .count()
}

/// Produce a list of guesses for the size of the key that might've been used to
/// perform a XOR encryption on cipher.
///
/// # Arguments
/// * `cipher` - The cipher text
/// * `sample_size` - the number of blocks which are sampled to calculate hamming-distance for
///                   finding repeating strings
/// * `max_key_length` - maximum key-length which is used for sampling.
///
/// # Panics
/// 1. If `max_key_length * sample_size > cipher.len()`, this function will panic because of
///    out-of-index error which will be caused when attempting to take a chunk of `max_key_size`
///    from cipher-text which isn't long enough.
pub fn guess_key_size(cipher: &[u8], sample_size: usize, max_key_length: usize) -> Vec<usize> {
    let mut edit_distances: Vec<(usize, usize)> = vec![];

    for key_size in 2..=max_key_length {
        let max_index = key_size * sample_size;
        if max_index > cipher.len() {
            panic!("Keysize out of bounds. Please reduce max key-size or chunk-size");
        }

        let mut chunks: Vec<&[u8]> = vec![&cipher[0..key_size]];
        for cn in 2..=sample_size {
            chunks.push(&cipher[key_size..(key_size * cn)]);
        }

        let edit_distance: usize = chunks
            .clone()
            .into_iter()
            .cartesian_product(chunks.into_iter())
            .map(|(c1, c2)| hamming_distance(c1, c2))
            .sum();
        let edit_distance = edit_distance / sample_size;

        let normalized_edit_distance = edit_distance / key_size;

        edit_distances.push((key_size, normalized_edit_distance));
    }

    edit_distances.sort_unstable_by(|(_, d1), (_, d2)| d1.cmp(d2));

    // println!("Edit distances: {:?}", edit_distances);

    edit_distances[0..4]
        .iter()
        .map(|(k, _)| *k)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_distance_for_given_example() {
        let s1 = "this is a test";
        let s1 = s1.as_bytes();
        let s2 = "wokka wokka!!!";
        let s2 = s2.as_bytes();

        assert_eq!(hamming_distance(s1, s2), 37);
    }

    #[test]
    fn test_guess_key_size() {
        let plain_text = "How do you do. I am well thank you. Some random message. Cat has blue eyes.";
        let key = "water";
        let key_size = key.as_bytes().len();

        let cipher = xor(plain_text.as_bytes(), key.as_bytes());

        let guessed_key_sizes = guess_key_size(&cipher);

        println!(
            "Actual Key size: {}. Guessed key sizes: {:?}",
            key_size, guessed_key_sizes
        );

        assert!(guessed_key_sizes.contains(&key_size));
    }
}
