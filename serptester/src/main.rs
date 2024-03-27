use clap::Parser;
use webbrowser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

enum Environment {
    Local,
    Stage,
    Prod,
}

enum PageType {
    Hosted,
    Embedded,
    Forwarding,
    Api,
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

    page_type: String,
}

const SERP_PATH: &str = "/serp?q=hotels&debug=show";
const EMBEDDED_DEMO_PATH: &str = "/embedded-demo?q=hotels&debug=show#S1&debug=1";

fn build_path(page_type: PageType) -> String {
    match page_type {
        PageType::Hosted => SERP_PATH.to_string(),
        PageType::Embedded => EMBEDDED_DEMO_PATH.to_string(),
        _ => SERP_PATH.to_string(),
    }
}

fn build_local_domain(domain: &str, page_type: PageType) -> String {
    format!("http://local.{}:8080{}", domain, build_path(page_type))
}

fn build_prod_domain(domain: &str, page_type: PageType) -> String {
    match page_type {
        PageType::Hosted => format!("https://{}{}", domain, build_path(page_type)),
        _ => format!("https://{}{}", domain.replace(".", "-"), build_path(page_type)),
    }
}

fn build_stage_domain(domain: &str, unit: Option<String>, page_type: PageType) -> String {
    let normailzed_domain = match page_type {
        PageType::Hosted => format!("{}", domain),
        _ => format!("{}", domain.replace(".", "-")),
    };

    match unit {
        None => format!("http://{}.search-stage.system1.company{}", normailzed_domain, build_path(page_type)),
        Some(unit_name) => format!("http://{}.search-stage-{}.system1.company{}", normailzed_domain, unit_name, build_path(page_type)),
    }
}

fn build_url(settings: &Args) -> String {
    let env: Environment = match settings.env.as_str() {
        "l" | "local" => Environment::Local,
        "s" | "stage" => Environment::Stage,
        _ => Environment::Prod,
    };

    let page_type: PageType = match settings.page_type.as_str() {
        "h" | "hosted" => PageType::Hosted,
        "e" | "embedded" => PageType::Embedded,
        "f" | "forwarding" => PageType::Forwarding,
        "a" | "api" => PageType::Api,
        _ => PageType::Hosted,
    };

    let unit_option: Option<String> = match settings.unit.as_str() {
        "X" => None,
        name => Some(name.to_string()),
    };  
    
    let domain = &settings.domain;
    match env {
        Environment::Local => build_local_domain(domain, page_type),
        Environment::Stage => build_stage_domain(domain, unit_option, page_type),
        Environment::Prod => build_prod_domain(domain, page_type),
    }
}

fn show_settings(settings: &Args) {
    println!("domain given is: {}", settings.domain);
    let environment = match settings.env.as_str() {
        "l" | "local" => "local",
        "s" | "stage" => "stage",
        _ => "defaulting to prod"
    };
    println!("environment: {}", environment);
    let page_type = match settings.page_type.as_str() {
        "h" | "hosted" => "hosted",
        "e" | "embedded" => "embedded",
        "f" | "forwarding" => "forwarding",
        "a" | "api" => "api",
        _ => "none given, defaulting to 'hosted'",
    };
    println!("page type: {}", page_type);

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

    show_settings(&args);
    let url = build_url(&args);
    println!("I will navigate to: {}", &url);
    if webbrowser::open(&url).is_ok() {
        println!("Look in your default Browser");
    }
}
