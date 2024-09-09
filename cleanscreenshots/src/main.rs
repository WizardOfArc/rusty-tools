use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fs;

const SCREEN_CAST_START: &str = "Screen Recording ";
const SCREEN_CAST_END: &str = ".mov";
const SCREEN_SHOT_START: &str = "Screenshot ";
const SCREEN_SHOT_END: &str = ".png";

fn remove_screenshots_in_dir(dir: &Path) {
    let dir_string = dir.to_str().unwrap();
    let mut files_to_remove: Vec<PathBuf> = Vec::new();
    let screenshot_prefix = format!("{}/{}", dir_string, SCREEN_SHOT_START);
    let screencast_prefix = format!("{}/{}", dir_string, SCREEN_CAST_START);
    match fs::read_dir(dir) {
        Err(error) => println!("Something weird happened {:?}", error),
        Ok(ls_iterator) => {
            for entry in ls_iterator {
                match entry {
                    Err(error) => println!("{:?}", error),
                    Ok(dir_entry) => {
                       let entry_path = dir_entry.path();
                       let filename = entry_path.to_str().unwrap();
                       if (filename.starts_with(&screencast_prefix) && filename.ends_with(SCREEN_CAST_END)) ||  (filename.starts_with(&screenshot_prefix) && filename.ends_with(SCREEN_SHOT_END)) {
                           files_to_remove.push(entry_path.clone());
                       }
                    },
                }
            }
        }
    }
    if files_to_remove.len() == 0 {
        println!("There are none to remove.");
    }
    for path in files_to_remove.iter() {
       println!("removing {:?}...", path);
       let _ = std::fs::remove_file(path);
    }

}

fn main() {
    match env::var("HOME"){
      Err(_) => println!("Please set HOME to your home directory"),
      Ok(home) => {
          let desktop_path_string =format!("{home}/Desktop");
          let desktop_path = Path::new(&desktop_path_string);
          match env::set_current_dir(desktop_path) {
            Err(error) => println!("couldn't change directories: {:?}", error),
            Ok(_) => {
               remove_screenshots_in_dir(desktop_path); 
            },
          }
      }
    }
}
