use rpassword::read_password;
use std::collections::HashSet;

const ASCII: i64 = 95;

const WORDS_BYTES: &str = include_str!("words.txt");

//const WORDS: HashSet<String> = load();

fn load() -> HashSet<String> {
    let words: HashSet<String> = WORDS_BYTES
        .split_whitespace()
        .map(|string| string.to_lowercase())
        .filter(|string| {
            !string.contains('-')
                && string.len() >= 3
                && string.chars().all(|char| char.is_ascii_alphabetic())
        })
        .collect();
    words
}

fn basic() -> i64 {
    let password: String = read_password().unwrap();
    let chars: Vec<char> = password.chars().collect();

    let basic_entropy: i64 = chars.len() as i64 * ASCII.ilog2() as i64;

    println!(
        "your input was {}, which is {:?} bits before any other checks",
        password, basic_entropy
    );

    basic_entropy
}

fn advanced(basic: &i64) -> i64 {
    let entropy: i64 = basic;

    entropy
}

fn main() {
    load();
    println!("input password");
    let mut basic: i64 = basic();
    let advanced: i64 = advanced(&basic);
}
