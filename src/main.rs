mod utils;

fn main() {
    let path = utils::parse_args();
    println!("{}", path)
}
