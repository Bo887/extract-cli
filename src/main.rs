mod commands;
mod utils;
mod errors;

#[macro_use]
extern crate lazy_static;

fn main() {
    let path = utils::parse_args();

    let error = utils::run_command(&path);
    if error.is_some() {
        println!("{:?}", error)
    }
}
