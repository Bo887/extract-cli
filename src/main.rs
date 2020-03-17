mod commands;
mod errors;
mod utils;

#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::Write;

fn main() {
    let path = utils::parse_args();

    let (err, stdout, stderr) = utils::run_command(&path);

    if err.is_some() {
        eprintln!("Error: {}", err.unwrap());
    } else {
        eprintln!("Successfully extracted {}", path);
    }
    if stdout.is_some() {
        println!("-------Stdout--------");
        io::stdout().write_all(&stdout.unwrap());
    }
    if stderr.is_some() {
        eprintln!("-------Stderr--------");
        io::stderr().write_all(&stderr.unwrap());
    }
}
