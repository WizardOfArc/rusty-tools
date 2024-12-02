use chrono::prelude::*;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// filename to make summary from
    filename: String,
}

const OPENING: &str = ",----------------------------------------,";
const CLOSING: &str = "`----------------------------------------'";
fn validate_filename(filename: &str) -> Result<NaiveDate, String> {
    if filename.ends_with("_update.txt") {
        if filename.len() != 19 {
            return Err("filename must be 19 characters long".to_string());
        } else {
            let date = &filename[0..8];
            match NaiveDate::parse_from_str(date, "%Y%m%d") {
                Ok(parsed_date) => Ok(parsed_date),
                Err(_) => {
                    Err("filename must start with a date in YYYYMMDD format".to_string())
                }
            }
        }
    } else {
        Err("filename must end with _update.txt".to_string())
    }
}

fn make_summary_filename(parsed_date: &NaiveDate) -> String {
    format!("{}_summary_update.txt", parsed_date.format("%Y%m%d"))
}

fn make_header(parsed_date: &NaiveDate) -> String {
    let date_bit = parsed_date.format("%Y/%m/%d");
    let date_line = format!("| SearchSites Team Update for {} |", date_bit);
    format!("{}\n{}\n{}\n\n", OPENING, date_line, CLOSING)
}

fn main() {
    let args = Args::parse();
    let team_update_dir = std::env::var("TEAM_UPDATE_DIR").unwrap();
    std::env::set_current_dir(team_update_dir).unwrap();
    match validate_filename(&args.filename) {
        Ok(parsed_date) => {
            let summary_filename = make_summary_filename(&parsed_date);
            let mut summary_file = File::create(summary_filename).unwrap();
            let header = make_header(&parsed_date);
            summary_file.write_all(header.as_bytes()).unwrap();
            let mut update_file = File::open(args.filename).unwrap();
            let mut update_contents = String::new();
            update_file.read_to_string(&mut update_contents).unwrap();
            summary_file.write_all(update_contents.as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
