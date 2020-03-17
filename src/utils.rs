extern crate clap;

use clap::{App, Arg};
use std::path::Path;
use std::process::Command;

use crate::commands;
use crate::errors;

pub fn parse_args() -> String {
    let matches = App::new("extract")
        .version("0.1.0")
        .author("Eric Zhang <ezhang887@gmail.com>")
        .arg(
            Arg::with_name("input_path")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("path of file to extract"),
        )
        .get_matches();
    let input_path = matches.value_of("input_path").unwrap();
    input_path.to_string()
}

fn get_command(path: &str) -> Result<String, errors::Error> {
    if !Path::new(&path).exists() {
        return Err(errors::Error::NoSuchFile);
    }

    for (extension, command) in &*commands::MAPPING {
        if path.ends_with(extension) {
            return Ok(format!("{} {}", command, path));
        }
    }

    Err(errors::Error::UnrecognizedExtension)
}

pub fn run_command(path: &str) -> Option<errors::Error> {
    let res = get_command(path);
    if res.is_err() {
        return res.err()
    }
    let command = res.unwrap();
    let args: Vec<&str> = command.split(" ").collect();
    let status = Command::new(args[0])
        .args(&args[1..])
        .status();
    if status.is_err() || !status.unwrap().success() {
        return Some(errors::Error::UnableToExtract);
    }
    None
}
