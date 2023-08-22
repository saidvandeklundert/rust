mod config_converter;
use log::info;
fn main() {
    println!("Hello, world!");
    env_logger::init();
    config_converter::debug();

    info!("Hello world!");
}
