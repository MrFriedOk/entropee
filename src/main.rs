use rpassword::read_password;
use std::collections::HashSet;
use std::{env, fs};

const ASCII: i64 = 95;

const WORDS_BYTES: &str = include_str!("words.txt");

const WORDS: HashSet<String> = load();

const fn load() -> HashSet<String> {}

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
    println!("input password");
    let basic: i64 = basic();
}
