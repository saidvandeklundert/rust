use std::fs;
use std::io::prelude::*;
use std::path::Path;

// Open a file and print the content to console:
pub fn display_file_content(file_path: &str) {
    println!("Opening file:");
    let contents: String = fs::read_to_string(file_path).expect("cannot open file!");
    println!("Read the file {file_path}:\n{contents}");
}

pub fn open_file_by_example(file_path: &str) {
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("could not read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    // file goes out of scope and is closed
}
