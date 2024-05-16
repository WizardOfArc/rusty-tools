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
         .append(true)
         .open(&args.outfile)
         .unwrap();
    let mut file_a = OpenOptions::new().read(true).open(&args.filename_a).unwrap();
    let mut file_b = OpenOptions::new().read(true).open(&args.filename_b).unwrap();
    let mut contents_a = String::new();
    let mut contents_b = String::new();
    file_a.read_to_string(&mut contents_a).unwrap();
    file_b.read_to_string(&mut contents_b).unwrap();
    let set_a: HashSet<String> = contents_a.lines().map(|s| s.to_string()).collect();
    let set_b: HashSet<String> = contents_b.lines().map(|s| s.to_string()).collect();
    let result = match args.operation {
        SetOperation::Union => set_union(&set_a, &set_b),
        SetOperation::Intersection => set_intersection(&set_a, &set_b),
        SetOperation::Difference => set_difference(&set_a, &set_b),
    };
    for line in result {
        writeln!(out_file, "{}", line).unwrap();
    }
}
