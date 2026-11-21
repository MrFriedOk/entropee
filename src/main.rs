use rpassword::read_password;

const ASCII: i64 = 95;

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
    let basic: i64 = basic();
}
