mod commands;
mod errors;
mod utils;

#[macro_use]
extern crate lazy_static;

fn main() {
    let path = utils::parse_args();

    let (err, stdout, stderr) = utils::run_command(&path);
    if err.is_some() {
        eprintln!("Error: {:?}", err.unwrap());
    }
}
