use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file with list of lines
    file: String,

    /// pattern to match
    pattern: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_matching_lines<P>(filename: P, pattern: &str) where P: AsRef<Path> {
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            if line.contains(pattern) {
                println!("{}", line);
            }
        }
    }
}

fn main() {
    let args: Args = Args::parse();
    let file = &args.file;
    let pattern = &args.pattern;
    println!("I'm looking for '{}' in {}", pattern, file);
    print_matching_lines(file, pattern);
}
