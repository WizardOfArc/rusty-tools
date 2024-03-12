use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time};

use clap::Parser;




// apply provided edit set to all domains in a provided list.

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///  file with list of domains
    domain_list_file: String,

    /// edit set file to apply
    edit_set: String,
}
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn apply_edit_set(domain: String, edit_set: &String) -> String {
    /*
    python tools/search_configs.py edit www.options.xyz ~/EditSets/delete_dpljs_version_edit_set.yaml
    */
    let output = 
        Command::new("python")
            .arg("tools/search_configs.py")
            .arg("edit")
            .arg(domain)
            .arg(edit_set)
            .output()
            .expect("failed to execute bulk edit");

   String::from_utf8(output.stdout).expect("Our bytes should be valid utf8")
}

fn apply_to_each_in_file<P>(filename: P, edit_set: String) where P: AsRef<Path> {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
            let domain = line.trim().to_string();
            apply_edit_set(domain, &edit_set);
            thread::sleep( time::Duration::from_secs(4));
        }
    }
}


fn main() {
    let args: Args = Args::parse();
    println!("Hello, you!");
    println!("domain list file: {}", args.domain_list_file);
    println!("edit set file: {}", args.edit_set);
    apply_to_each_in_file(args.domain_list_file, args.edit_set)
}



