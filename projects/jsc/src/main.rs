mod config_converter;

fn main() {
    println!("Hello, world!");
    env_logger::init();
    config_converter::debug();
}
