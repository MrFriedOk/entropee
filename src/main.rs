use rpassword::read_password;

const ASCII: i32 = 95;

fn main() {
    let password: String = read_password().unwrap();
    let chars: Vec<char> = password.chars().collect();
    let basic_entropy: i32 = ASCII.pow(chars.len() as u32);
    println!("{:?}", basic_entropy);
    //println!("Hello, world!");
}
