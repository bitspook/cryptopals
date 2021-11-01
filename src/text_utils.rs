fn common_chars_frequency(sentence: &str) -> usize {
    let most_frequent_chars = "Eaton";
    let mut freq = 0;
    for ch in sentence.chars() {
        if most_frequent_chars.contains(ch) {
            freq += 1;
        }
    }

    freq * 100 / sentence.len()
}

// At compile time, there should be a file named dict.txt in current directory,
// and it should have all words in lowercase
fn dictionary_words_frequency(sentence: &str) -> usize {
    let dict_words = include_str!("./dict.txt");
    let words = sentence.split_ascii_whitespace();
    let mut total_words = 0;
    let mut count = 0;

    for word in words {
        if dict_words.contains(&word.to_ascii_lowercase()) {
            count += 1;
        }

        total_words += 1;
    }

    count * 100 / total_words
}

pub fn is_plain_sentence(sentence: &str) -> bool {
    let common_char_freq = common_chars_frequency(sentence);
    let dict_words_freq = dictionary_words_frequency(sentence);

    common_char_freq > 25 && dict_words_freq > 70
}
