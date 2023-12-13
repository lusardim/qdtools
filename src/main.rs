use std::fs;
use walkdir::WalkDir;

fn main() {
    let root = ".";
    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            check_for_utf8(&entry.path().display().to_string());
        }
    }
}

fn check_for_utf8(file: &String) {
    let content = fs::read(file);
    match content {
        Ok(bytes) => {
            if let Err(error) = std::str::from_utf8(&bytes) {
                println!("Invalid utf-8 in file {} - {}", file, error);
            }
        }
        Err(error) => println!("Failed to read {} - {}", file, error),
    }
}
