use std::fs;

// Open a file and print the content to console:
pub fn display_file_content(file_path:&str) {
    println!("Opening file:");
    let contents: String = fs::read_to_string(file_path).expect("cannot open file!");
    println!("Read the file {file_path}:\n{contents}");
}