use std::{env, fmt};
use std::fs::read_to_string;
use std::io::stdin;
use std::path::Path;

use chrono::prelude::*;
use clap::Parser;

const TODO_SUPPORT_FILE: &str = "TODO_SUPPORT_FILE";

#[derive(clap::ValueEnum, Clone, Debug)]
enum Command {
    Show,
    Add,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    command: Command, 
}

struct TodoEntry {
    create_date: String, // in YYYYmmddhhmm form
    due_date: String, // in YYYYmmddhhmm form
    task_desciption: String,
}

impl TodoEntry {
    fn make_row(&self) -> String {
        format!("{}|>{}|>{}", self.create_date, self.task_desciption, self.due_date)
    }

    fn from_row(todo_row: String) -> Option<TodoEntry> {
        let pieces: Vec<&str> = todo_row.split("|>").collect();
        if pieces.len() != 3 {
            println!("skipping invalid todo row: {}", todo_row);
            None
        } else {
            Some(TodoEntry {
                create_date: pieces[0].to_string(),
                task_desciption: pieces[1].to_string(),
                due_date: pieces[2].to_string(),
            })
        }
    }
}

impl fmt::Display for TodoEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})\n{}\n[Due: {}]\n===\n", self.create_date, self.task_desciption, self.due_date)
    }
}

fn show_todo(filename: String) {
    // check if file exists if not - alert
    if !Path::new(&filename).exists() {
        println!("The file, {}, does not exist!", filename);
    } else {
        match read_to_string(filename) {
            Err(error) => { println!("Error reading file: {}", error) },
            Ok(contents) => {
                let todo_rows: Vec<&str> = contents.split("\n").collect();
                for row in todo_rows {
                    match TodoEntry::from_row(row.trim().to_string()){
                        None => {},
                        Some(todo) => { println!("{}", todo) },
                    }
                }
            }
        }
    }
}

fn add_todo(filename: String) {
    let now = Local::now();
    let now_string = now.format("%Y%m%d%H%M").to_string();
    let mut todo_entries: Vec<TodoEntry> = Vec::new();
    if Path::new(&filename).exists() {
        match read_to_string(&filename) {
            Err(error) => { println!("Error reading file: {}", error) },
            Ok(contents) => {
                let todo_rows: Vec<&str> = contents.split("\n").collect();
                for row in todo_rows {
                    match TodoEntry::from_row(row.trim().to_string()){
                        None => {},
                        Some(todo) => { todo_entries.push(todo) },
                    }
                }
            }
        }
    }

    println!("Enter task description:");
    let mut description: String = String::new();
    stdin().read_line(&mut description).unwrap();

    println!("Enter due date (YYYYMMDDHHMM):");
    let mut due: String = String::new();
    stdin().read_line(&mut due).unwrap();

    let new_entry = TodoEntry {
        create_date: now_string,
        task_desciption: description.trim().to_string(),
        due_date: due.trim().to_string(),
    };
    todo_entries.push(new_entry);

    let contents = todo_entries.into_iter().map(|t| t.make_row()).collect::<Vec<String>>().join("");
    std::fs::write(&filename, contents).unwrap();
}

fn main() {
    match env::var(TODO_SUPPORT_FILE) {
        Err(error) => { panic!("TODO_SUPPORT_FILE env var needs to be set: {}", error) },
        Ok(filename) => {
           let args: Args = Args::parse();
            match args.command {
                Command::Show => show_todo(filename),
                Command::Add => add_todo(filename),
           }
        }
    }
}
