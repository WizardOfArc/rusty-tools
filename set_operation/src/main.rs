use std::fs::OpenOptions;
use std::collections::HashSet;
use std::io::prelude::*;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug, Clone, ValueEnum)]
enum SetOperation {
    Union,
    Intersection,
    Difference,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    operation: SetOperation,
    filename_a: String,
    filename_b: String,
    outfile: String,
}

fn set_union(set_a: &HashSet<String>, set_b: &HashSet<String>) -> HashSet<String> {
    set_a.union(set_b).cloned().collect()
}

fn set_intersection(set_a: &HashSet<String>, set_b: &HashSet<String>) -> HashSet<String> {
    set_a.intersection(set_b).cloned().collect()
}

fn set_difference(set_a: &HashSet<String>, set_b: &HashSet<String>) -> HashSet<String> {
    set_a.difference(set_b).cloned().collect()
}

fn main() {
     let args = Args::parse();
     let mut out_file = OpenOptions::new()
         .write(true)
         .create(true)
         .append(false)
         .open(&args.outfile)
         .expect("Could not open output file");
    let mut file_a = OpenOptions::new().read(true).open(&args.filename_a).expect("Could not open file A");
    let mut file_b = OpenOptions::new().read(true).open(&args.filename_b).expect("Could not open file B");
    let mut contents_a = String::new();
    let mut contents_b = String::new();
    file_a.read_to_string(&mut contents_a).expect("Could not read file A");
    file_b.read_to_string(&mut contents_b).expect("Could not read file B");
    let set_a: HashSet<String> = contents_a.lines().map(|s| s.to_string()).collect();
    let set_b: HashSet<String> = contents_b.lines().map(|s| s.to_string()).collect();
    let result = match args.operation {
        SetOperation::Union => set_union(&set_a, &set_b),
        SetOperation::Intersection => set_intersection(&set_a, &set_b),
        SetOperation::Difference => set_difference(&set_a, &set_b),
    };
    let mut result: Vec<String> = result.iter().cloned().collect();
    result.sort_by_key(|s| s.to_lowercase());
    let output  = result.join("\n");
    out_file.write_all(output.as_bytes()).expect("Could not write to output file");
}
