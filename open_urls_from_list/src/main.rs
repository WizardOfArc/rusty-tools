use std::fs::File;
use std::io::{self, stdin, BufRead};
use std::path::Path;
use webbrowser;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Let's looks at some web pages!");
    let mut url_list_file_path = String::new();
    println!("What's the file with the list of urls?");
    stdin().read_line(&mut url_list_file_path).expect("Failed to read line");
    let mut proceed: String = String::new();
    let read_lines_result = read_lines(url_list_file_path.trim());
    match read_lines_result {
        Ok(lines) => {
            for line in lines {
                let url = line.unwrap();
                println!("I will navigate to: {}", &url);
                if webbrowser::open(&url).is_ok() {
                    println!("Look in your default Browser");
                } else {
                    println!("Failed to open {}", &url);
                }
                println!("<enter> to continue, 'q' to quit");
                stdin().read_line(&mut proceed).expect("Failed to read line");
                if proceed.trim() == "q" {
                    break;
                } 
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    };
}
