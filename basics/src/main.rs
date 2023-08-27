//mod comp_errors;
#[allow(dead_code)]
mod strings;
mod working_with_files;
fn main() {
    println!("Hello, world!");
    working_with_files::display_file_content("poem.txt");
    working_with_files::open_file_by_example("poem.txt");
    strings::strings_example();
}
