use rpassword::read_password;

const ASCII: u32 = 95;

fn main() {
    let password: String = read_password().unwrap();
    let chars: Vec<char> = password.chars().collect();
    let basic_comb: u32 = ASCII.pow(chars.len() as u32);
    let basic_entropy: u32 = basic_comb.ilog2();
    println!("{:?}", basic_entropy);
    //println!("Hello, world!");
}
