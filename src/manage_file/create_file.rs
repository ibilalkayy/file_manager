use std::fs::File;
use std::path::Path;
use crate::utils::utils::ask_file;

pub fn create_file(input: &mut String) {
    print!("Enter the file path to create: ");
    let file_path = ask_file(input);
    let path = Path::new(input);
    let display = path.display();

    match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    println!("The file '{:?}' is successfully created", file_path);
}