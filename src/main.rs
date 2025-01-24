use passgen::generate_password;

fn main() {
    let password = generate_password();
    println!("{}", password);
}
