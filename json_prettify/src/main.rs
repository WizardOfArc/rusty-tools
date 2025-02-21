use clap::Parser;
use std::error::Error;
use std::fs;

/// Simple program to prettify JSON files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input JSON file path
    #[arg(short, long)]
    input: String,

    /// Output JSON file path
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Read the input JSON file
    let json_content = fs::read_to_string(&args.input)?;

    // Parse and prettify the JSON
    let json_value: serde_json::Value = serde_json::from_str(&json_content)?;
    let pretty_json = serde_json::to_string_pretty(&json_value)?;

    // Write to output file
    fs::write(&args.output, pretty_json)?;

    println!("JSON has been prettified and saved to {}", args.output);
    Ok(())
}