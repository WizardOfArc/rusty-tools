use std::env;
use std::fs::File;
use std::io::{self, stdin, BufRead};
use std::path::Path;
use std::process::exit;
use clap::builder::Str;
use clap::Parser;
use webbrowser;

enum Environment {
    Local,
    Stage,
    Prod,
}

#[derive(Debug)]
enum PageType {
    Hosted,
    Embedded,
    Forwarding,
    Api,
    Unknown,
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

fn find_domain_type(domain: &str) -> PageType {
    let domain_list_file: String = format!("{}/openmail_common/search/config.py", env::var("S1HOME").unwrap() );
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
    match domain_type {
        PageType::Unknown => {
            println!("Domain '{}' not found", domain);
            exit(1);
        },
        _ => (),
    }
    domain_type
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// domain to check
    domain: String,

    /// environment: local, stage, prod or l,s,p
    env: String,
    
    /// stage unit - if not on stage or is main stage - use 'X'
    unit: String,
}

const SERP_PATH: &str = "/serp?q=hotels&debug=show";
const EMBEDDED_DEMO_PATH: &str = "/embedded-demo?q=hotels&debug=show#S1&debug=1";

fn build_path(page_type: &PageType) -> String {
    match page_type {
        PageType::Hosted => SERP_PATH.to_string(),
        PageType::Embedded => EMBEDDED_DEMO_PATH.to_string(),
        _ => SERP_PATH.to_string(),
    }
}

fn build_local_domain(domain: &str, page_type: &PageType) -> String {
    format!("http://local.{}:8080{}", domain, build_path(page_type))
}

fn build_prod_domain(domain: &str, page_type: &PageType) -> String {
    let path = build_path(page_type);
    let vanity_suffix = ".s1search.co";
    let prod_domain = match page_type {
        PageType::Hosted => {
            format!("https://{}{}", domain, path)
        },
        PageType::Embedded => {
            let formatted_domain = domain.replace(".", "-");
            format!("https://{}{}{}", formatted_domain, vanity_suffix, path)
        },
        _ => {
            format!("https://{}{}.s1search.co", domain.replace(".", "-"), build_path(page_type))
        },
    };
    println!("prod domain built: {:?}", &prod_domain);
    return prod_domain;
}

fn build_stage_domain(domain: &str, unit: Option<String>, page_type: &PageType) -> String {
    let normailzed_domain = match page_type {
        PageType::Hosted => format!("{}", domain),
        _ => format!("{}", domain.replace(".", "-")),
    };

    match unit {
        None => format!("http://{}.search-stage.system1.company{}", normailzed_domain, build_path(page_type)),
        Some(unit_name) => format!("http://{}.search-stage-{}.system1.company{}", normailzed_domain, unit_name, build_path(page_type)),
    }
}

fn build_url(settings: &Args, page_type: &PageType) -> String {
    let env: Environment = match settings.env.as_str() {
        "l" | "local" => Environment::Local,
        "s" | "stage" => Environment::Stage,
        _ => Environment::Prod,
    };

    let domain = &settings.domain;

    let unit_option: Option<String> = match settings.unit.as_str() {
        "X" => None,
        name => Some(name.to_string()),
    };  
    
    match env {
        Environment::Local => build_local_domain(domain, page_type),
        Environment::Stage => build_stage_domain(domain, unit_option, page_type),
        Environment::Prod => build_prod_domain(domain, page_type),
    }
}

fn show_settings(settings: &Args, page_type: &PageType) {
    println!("domain given is: {}", settings.domain);
    let environment = match settings.env.as_str() {
        "l" | "local" => "local",
        "s" | "stage" => "stage",
        _ => "defaulting to prod"
    };
    println!("environment: {}", environment);
    println!("page type: {:?}", page_type);

    let unit = match settings.unit.as_str() {
        "X" => "none (X was entered)",
        name => match settings.env.as_str() {
            "s" | "stage" => name,
            _ => "none (this environment does not use units)",
        }
    };
    println!("unit name: {}", unit);
}

fn main() {
    println!("I am the SERP Tester!\nI'll help you navigate to your serp");
    let args = Args::parse();
    let domain = &args.domain;
    let page_type: PageType = find_domain_type(domain);
    show_settings(&args, &page_type);
    let url = build_url(&args, &page_type);
    println!("I will navigate to: {}", &url);
    let mut okay: String = String::new();
    println!("Enter to launch browser, or q to quit");
    stdin().read_line(&mut okay).expect("Failed to read line");
    match okay.trim() {
        "q" => exit(0),
        _ => (),
    }
    if webbrowser::open(&url).is_ok() {
        println!("Look in your default Browser");
    }
}
