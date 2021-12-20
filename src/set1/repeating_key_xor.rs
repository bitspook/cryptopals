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
pub fn guess_key_size(cipher: &[u8]) -> Vec<usize> {
    let mut nhds: Vec<(usize, f32)> = vec![];

    for partitions in (2..).step_by(2) {
        let sample_size = partitions / 2;
        let key_size = cipher.len() / partitions;

        if key_size <= 2 {
            break;
        }

        let max_index = key_size * sample_size;
        if max_index > cipher.len() {
            panic!("Keysize out of bounds. Please reduce max key-size or sample-size");
        }

        let mut chunks: Vec<&[u8]> = vec![&cipher[0..key_size]];
        for cn in 2..=partitions {
            chunks.push(&cipher[key_size..(key_size * cn)]);
        }

        let mut edit_distances: Vec<usize> = vec![];
        for i in (0..chunks.len()).step_by(2) {
            edit_distances.push(hamming_distance(chunks[i], chunks[i + 1]));
        }

        let edit_distance: usize = edit_distances.into_iter().sum();
        let avg_edit_distance = edit_distance as f32 / chunks.len() as f32;
        let normalized_edit_distance = avg_edit_distance / key_size as f32;

        nhds.push((key_size, normalized_edit_distance));
    }

    nhds.sort_unstable_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

    println!("Edit distances: {:?}", nhds);

    nhds.iter().map(|(k, _)| *k).collect()
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
