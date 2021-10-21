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

pub fn is_plain_sentence(sentence: &str) -> bool {
    if !has_valid_ascii_chars(sentence)
        || !has_compulsary_chars(sentence)
        || has_forbidden_digraphs(sentence)
    {
        return false;
    }

    let most_frequent_chars = "Eaton";
    let mut freq = 0;
    for ch in sentence.chars() {
        if most_frequent_chars.contains(ch) {
            freq += 1;
        }
    }

    let freq_percent = freq * 100 / sentence.len();

    freq_percent > 28
}
