extern crate clap;

use clap::{Arg, App};

pub fn parse_args() -> String {
    let matches = App::new("extract")
        .version("0.1.0")
        .author("Eric Zhang <ezhang887@gmail.com>")
        .arg(Arg::with_name("path")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("path of file to extract"))
        .get_matches();
    let path = matches.value_of("path").unwrap();
    path.to_string()
}
