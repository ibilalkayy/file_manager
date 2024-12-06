use std::io::Write;
use std::process::exit;
use crate::manage_file::{
    create_file::create_file,
    copy_file::copy_file,
    move_file::move_file,
    rename_file::rename_file,
};

fn ask(input: &mut String) -> u32 {
    println!("1. Create a file");
    println!("2. Copy a file");
    println!("3. Move a file");
    println!("4. Rename a file");
    println!("5. Exit");

    print!("Enter your choice: ");
    std::io::stdout().flush().unwrap();
    input.clear();

    std::io::stdin().read_line(input).unwrap();
    input.trim().parse().expect("Expected a number")
}

pub fn select(
    input_one: &mut String,
    first_file_path: &mut String,
    second_file_path: &mut String,
) {
    let choice = ask(input_one);
    match choice {
        1 => create_file(first_file_path),
        2 => copy_file(first_file_path, second_file_path),
        3 => move_file(first_file_path, second_file_path),
        4 => rename_file(first_file_path),
        5 => {
             println!("Exited");
             exit(0);
        },
        _ => println!("Invalid choice"),
    }
}

pub fn ask_file(file_path: &mut String) -> String{
    std::io::stdout().flush().unwrap();
    file_path.clear();

    std::io::stdin().read_line(file_path).unwrap();
    file_path.trim().to_string()
}