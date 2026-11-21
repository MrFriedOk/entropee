use rpassword::read_password;

fn main() {
    let password: String = read_password().unwrap();
    let chars: Vec<char> = password.chars().collect();
    println!("{:?}", &chars);
    //println!("Hello, world!");
}
