use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::Parser;

enum PageType {
    Hosted,
    Embedded,
    Forwarding,
    Api,
    Unknown,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// domain to check
    domain: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_domain_type_from_line(line: String) -> PageType {
    let mut domain_type = PageType::Unknown;
    if line.contains("hosted_partner") {
        domain_type = PageType::Hosted;
    } else if line.contains("embedded_partner") {
        domain_type = PageType::Embedded;
    } else if line.contains("forwarding_partner") {
        domain_type = PageType::Forwarding;
    } else if line.contains("api_partner") {
        domain_type = PageType::Api;
    }
    domain_type
}

fn find_domain_type(domain: &str, domain_list_file: String) -> PageType {
    let mut domain_type = PageType::Unknown;
    if let Ok(lines) = read_lines(domain_list_file) {
        for line in lines {
            let line = line.unwrap();
            if line.contains(domain) {
                domain_type = get_domain_type_from_line(line);
                match domain_type {
                    PageType::Hosted => break,
                    PageType::Embedded => break,
                    PageType::Forwarding => break,
                    PageType::Api => break,
                    PageType::Unknown => continue,
                }
            }
        }
    }
    domain_type
}

fn main() {
    let args: Args = Args::parse();
    let domain = &args.domain;
    let domain_list_file: String = format!("{}/openmail_common/search/config.py", env::var("S1HOME").unwrap() );
    let search_result = find_domain_type(domain, domain_list_file);
    match search_result {
        PageType::Hosted => println!("{} is a hosted domain", domain),
        PageType::Embedded => println!("{} is an embedded domain", domain),
        PageType::Forwarding => println!("{} is a forwarding domain", domain),
        PageType::Api => println!("{} is an api domain", domain),
        PageType::Unknown => println!("{} is an unknown domain", domain),
    }
}
