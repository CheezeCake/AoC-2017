use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn has_unique_words(words: &Vec<String>) -> bool {
    let mut word_set = HashSet::new();
    for word in words {
        if word_set.contains(word) {
            return false;
        }
        word_set.insert(word);
    }
    true
}

fn make_letter_count(s: &str) -> HashMap<u8, u32> {
    let mut letter_count = HashMap::new();
    for &c in s.as_bytes() {
        let cnt = letter_count.entry(c).or_insert(0);
        *cnt += 1;
    }
    letter_count
}

fn has_no_anagrams(words: &Vec<String>) -> bool {
    let mut letter_counts = Vec::new();
    for word in words {
        let letter_count = make_letter_count(word);
        if letter_counts.iter().any(|m| m == &letter_count) {
            return false;
        }
        letter_counts.push(letter_count);
    }
    true
}

fn main() {
    let reader = BufReader::new(io::stdin());

    let passphrases: Vec<Vec<String>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|words| words.to_string())
                .collect()
        })
        .collect();

    let mut unique_words = 0;
    let mut no_anagrams = 0;
    for words in passphrases {
        if has_unique_words(&words) {
            unique_words += 1;
        }
        if has_no_anagrams(&words) {
            no_anagrams += 1;
        }
    }

    println!("part 1: {}", unique_words);
    println!("part 2: {}", no_anagrams);
}
