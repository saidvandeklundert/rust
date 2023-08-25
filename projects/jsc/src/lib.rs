// RUST_LOG=info cargo run
// RUST_LOG=debug cargo run
// cargo test
use log::{debug, info, warn};
use std::fmt::Display;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
mod configwriter;
mod lexer;
mod utils;
use crate::configwriter::ConfigWriter;
use crate::lexer::{Lexer, Token};
use crate::utils::{build_string, open_config_file};

pub fn debug() {
    info!("In the debug, running lib!");
    let file_name = "config_17";
    let config_file = file_name.to_owned() + ".txt";
    let expected_file = file_name.to_owned() + "_set.txt";
    let config = utils::open_config_file(&config_file);

    let mut config_writer = ConfigWriter::new(config);
    let config_writer_result = config_writer.write_configs();
    println!("Output:\n\n{config_writer_result}");
    let expected = open_config_file(&expected_file);
    println!(
        "\nShould be:\n
{expected}\n\n"
    );

    assert_eq!(config_writer_result, expected);
}
