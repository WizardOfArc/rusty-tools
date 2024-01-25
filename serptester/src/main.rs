use clap::Parser;
use webbrowser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// domain to check
    domain: String,

    // environment: local, stage, prod or l,s,p
    env: String,
    
    /// stage unit - if not on stage or is main stage - use 'X'
    unit: String,

    page_type: String,
}

const SERP_PATH: &str = "/serp?q=hotels&debug=show";
const EMBEDDED_DEMO_PATH: &str = "/embedded-demo?q=hotels&debug=show#S1&debug=1";


fn build_url(settings: &Args) -> String {
    let host_root: String = match settings.env.as_str() {
        "l" | "local" => format!("local.{}", settings.domain),
        _ => match settings.page_type.as_str() {
            "h" | "hosted" => format!("{}", settings.domain),
            _ => settings.domain.replace(".", "-"),
        }
    };
    let host_suffix: String = match settings.env.as_str() {
        "l" | "local" => format!("{}", ":8080"),
        "s" | "stage" => match settings.unit.as_str() {
            "X" => format!("{}",  ".search-stage.system1.company"),
            _ => format!(".search-stage-{}.system1.company", settings.unit),
        },
        _ => String::from("")
    };
    let path_string: String = match settings.page_type.as_str() {
        "e" | "embedded" => format!("{}", EMBEDDED_DEMO_PATH),
        _ => format!("{}", SERP_PATH),
    };
    format!("http://{}{}{}", host_root, host_suffix, path_string)

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
    println!("Hello, world!");
    let args = Args::parse();

    show_settings(&args);
    let url = build_url(&args);
    println!("I will navigate to: {}", &url);
    if webbrowser::open(&url).is_ok() {
        println!("Look in your default Browser");
    }
}
