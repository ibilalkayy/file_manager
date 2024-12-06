mod utils;
mod manage_file;

use crate::utils::utils::select;

fn main() {
    let (mut input, mut first_file_path, mut second_file_path): (String, String, String) = (String::new(), String::new(), String::new());
    println!("Welcome to the file manager application!");
    select(&mut input, &mut first_file_path, &mut second_file_path);
}