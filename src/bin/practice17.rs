use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, usize> {
    let words = text
        .split_whitespace()
        .map(|x| x.to_lowercase())
        .collect::<Vec<String>>();

    let mut word_count = HashMap::new();
    for word in words {
        *word_count.entry(word).or_insert(0) += 1;
    }

    word_count
}

fn main() {
    let text = "the cat and the dog and the bird";
    let counts = count_words(text);
    let mut pairs = counts.iter().collect::<Vec<(&String, &usize)>>();
    pairs.sort_by(|a, b| b.1.cmp(a.1));

    for (word, count) in pairs {
        println!("{}: {}", word, count);
    }
}
