use rpassword::read_password;

const ASCII: i64 = 95;

fn main() {
    let password: String = read_password().unwrap();
    let chars: Vec<char> = password.chars().collect();

    let basic_entropy: i64 = chars.len() as i64 * ASCII.ilog2() as i64;

    println!("{:?}", basic_entropy);
    //println!("Hello, world!");
}
