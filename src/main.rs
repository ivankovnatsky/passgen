use passgen::generate_password;

const HELP: &str = "\
passgen - generate a memorable random password

USAGE:
    passgen [OPTIONS]

OPTIONS:
    -h, --help       Print this help message
    -V, --version    Print version information

Generates a password in the format xxxxxx-xxxxxx-xxxxxx, where each
character is a lowercase/uppercase letter or digit.";

fn main() {
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{}", HELP);
                return;
            }
            "-V" | "--version" => {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                return;
            }
            other => {
                eprintln!("error: unexpected argument '{}'", other);
                eprintln!("\n{}", HELP);
                std::process::exit(1);
            }
        }
    }

    let password = generate_password();
    println!("{}", password);
}
