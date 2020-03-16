mod utils;
mod commands;

fn main() {
    let path = utils::parse_args();

    println!("{}", path);
}
