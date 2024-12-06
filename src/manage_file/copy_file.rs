use std::fs;
use std::path::Path;
use crate::utils::utils::ask_file;

pub fn copy_file(file_input: &mut String, folder_input: &mut String) {
    print!("Enter the file name to copy: ");
    let file_name = ask_file(file_input);
    let file_path = Path::new(&file_name);

    if !file_path.exists() || !file_path.is_file() {
        println!("File not found or is not a valid file.");
        return;
    }

    print!("Enter the folder path to copy to: ");
    let folder_name = ask_file(folder_input);
    let folder_path = Path::new(&folder_name);

    if !folder_path.exists() || !folder_path.is_dir() {
        println!("Folder not found or is not a valid directory.");
        return;
    }

    let file_name_only = file_path.file_name().unwrap(); 
    let destination = folder_path.join(file_name_only); 

    match fs::copy(&file_path, &destination) {
        Ok(_) => {
            println!("File copied successfully to: {}", destination.display());
        }
        Err(e) => {
            println!("Failed to copy file: {}", e);
        }
    }
}
