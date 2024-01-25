use clap::Parser;
use webbrowser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Web page to go to
    href: String,
}

fn main() {
    let args = Args::parse();
    println!("Lemme open {} for you", &args.href);
    if webbrowser::open(&args.href).is_ok() {
        println!("Look in your default Browser");
    }
}
