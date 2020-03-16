extern crate clap;

use clap::{App, Arg};

use std::ffi::OsStr;
use std::path::Path;

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

pub fn get_extension(path: &str) -> Option<&str> {
    Path::new(path).extension().and_then(OsStr::to_str)
}
