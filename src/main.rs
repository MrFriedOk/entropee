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

fn basic(password: &String) -> i64 {
    let chars: Vec<char> = password.chars().collect();

    let basic_entropy: i64 = chars.len() as i64 * ASCII.ilog2() as i64;

    println!(
        "your input was {}, which is {:?} bits before any other checks",
        password, basic_entropy
    );

    basic_entropy
}

fn advanced(basic: &i64, password: &String, words: &HashSet<String>) {
    let mut entropy: i64 = *basic;

    let words_in: Vec<&String> = words
        .iter()
        .filter(|w| password.contains(*w))
        .collect::<Vec<_>>();
    //for word in &words.iter().collect::<Vec<_>>() {
    //    if password.contains(*word) {
    //        println!("your password has a known word in it")
    //    }
    //}

    entropy = words_in.len() as i64 * words.len().ilog2() as i64;

    println!(
        "original entropy was {}. after some checks, it is now {}\nthe words found were, {:?}",
        basic, entropy, words_in
    )

    //basic
}

fn main() {
    println!("loading, please wait");
    let words: HashSet<String> = load();
    println!("input password");

    let password: String = read_password().unwrap();

    let basic: i64 = basic(&password);

    advanced(&basic, &password, &words)
    //let advanced: i64 = advanced(&mut basic, &password, &words);
    //println!("after some checks, the entropy is now {} bits", advanced);
}
