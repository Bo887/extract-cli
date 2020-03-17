extern crate clap;

use clap::{App, Arg};
use std::path::Path;

use crate::commands;
use crate::errors;

pub fn parse_args() -> String {
    let matches = App::new("extract")
        .version("0.1.0")
        .author("Eric Zhang <ezhang887@gmail.com>")
        .arg(
            Arg::with_name("path")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("path of file to extract"),
        )
        .get_matches();
    let path = matches.value_of("path").unwrap();
    path.to_string()
}

pub fn get_command(path: &str) -> Result<&str, errors::Error> {
    if !Path::new(path).exists() {
        return Err(errors::Error::NoSuchFile);
    }

    for (extension, command) in &*commands::MAPPING {
        if path.ends_with(extension) {
            return Ok(command);
        }
    }

    Err(errors::Error::UnrecognizedExtension)
}
