fn has_valid_ascii_chars(sentence: &str) -> bool {
    sentence
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_whitespace())
}

fn has_forbidden_digraphs(sent: &str) -> bool {
    let forbidden_digraphs = [
        "cj", "fq", "gx", "hx", "jf", "jq", "jx", "jz", "qb", "qc", "qj", "qk", "qx", "qz", "sx",
        "vf", "vj", "vq", "vx", "wx", "xj", "zx",
    ];

    for digraph in forbidden_digraphs {
        return sent.contains(digraph);
    }

    println!("NOT A SENTENCE: {} ", sent);
    return false;
}

fn has_compulsary_chars(sent: &str) -> bool {
    let compulsary_chars = "aeiouy".chars();

    for ch in compulsary_chars {
        if sent.contains(ch) {
            return true;
        }
    }

    false
}

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
    if !has_valid_ascii_chars(sentence)
        || !has_compulsary_chars(sentence)
        || has_forbidden_digraphs(sentence)
    {
        return false;
    }

    let common_char_freq = common_chars_frequency(sentence);

    if common_char_freq < 20 {
        return false;
    }

    let dict_words_freq = dictionary_words_frequency(sentence);
    dict_words_freq > 70
}
