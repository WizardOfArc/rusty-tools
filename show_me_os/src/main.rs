use std::env::consts::OS;


fn main() {
    println!("Hello, {} user!", OS);
    println!("You use '{}' as your separator", std::path::MAIN_SEPARATOR);
}
