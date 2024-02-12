use std::fs;
use std::path::Path;

fn main() {
    // Path to the directory containing desktop files
    let desktop_files_path = "/usr/share/applications";

    // Read the contents of the directory
    if let Ok(entries) = fs::read_dir(desktop_files_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Get the file path for each entry
                let file_path = entry.path();

                // Display the file name
                if let Some(file_name) = file_path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        println!("{}", file_name_str);
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {}", desktop_files_path);
    }
}

