mod commands;
mod utils;
mod errors;

#[macro_use]
extern crate lazy_static;

fn main() {
    let path = utils::parse_args();

    println!("{:?}", utils::get_command(path.as_str()));
}
