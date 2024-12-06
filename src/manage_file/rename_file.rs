use std::fs;
use std::path::Path;
use crate::utils::utils::ask_file;

pub fn rename_file(file_input: &mut String) {
    print!("Enter the file old name: ");
    let file_name = ask_file(file_input);
    let file_path = Path::new(&file_name);

    if !file_path.exists() || !file_path.is_file() {
        println!("File not found or is not a valid file.");
        return;
    }

    print!("Enter the file new name: ");
    let file_new_name = ask_file(file_input);

    match fs::rename(&file_name, file_new_name) {
        Ok(_) => {
            println!("Renamed the file successfully");
        }
        Err(e) => {
            println!("Faile to rename the file {}", e);
        }
    }
}