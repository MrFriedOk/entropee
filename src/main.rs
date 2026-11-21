use rpassword::read_password;
use std::collections::HashSet;
use std::{env, fs};

const ASCII: i64 = 95;

const WORDS_BYTES: &str = include_str!("words.txt");

//const WORDS: HashSet<String> = load();

fn load() {
    // -> HashSet<String> {
    let words: HashSet<String> = WORDS_BYTES
        .lines()
        .map(|word| word.to_lowercase())
        .filter(|word| word.contains('-'))
        .collect();
    println!("{:?}", words);

    //HashSet<"Hi">
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

fn main() {
    load();
    println!("input password");
    let basic: i64 = basic();
}
