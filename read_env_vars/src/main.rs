use std::env;
use std::io::stdin;

fn look_up_env_var() {
    let mut env_var_name: String = String::new();
    println!("What ENV var are you looking for?");
    stdin().read_line(&mut env_var_name).expect("Failed to read line");
    let env_var_name = env_var_name.trim();
    match env::var(env_var_name) {
        Ok(val) => println!("{}: {:?}", env_var_name, val),
        Err(e) => println!("couldn't find {}: {}", env_var_name, e),
    }
}

fn main() {
    println!("Well, hello there!");
    let mut proceed: String = String::new();
    loop {
        println!("Let's look up ENV var (q to quit, anything else to proceed)");
        stdin().read_line(&mut proceed).expect("Failed to read line");
        if proceed.trim() == "q" {
            break;
        }
        look_up_env_var();
    }
}
