use std::fs;

fn main() {
    let dot = fs::read_to_string("deps.dot").unwrap();

    for s in [
        diagvrs::Order::Dependencies,
        diagvrs::Order::Depth,
        diagvrs::Order::Alphabetical,
        diagvrs::Order::ReverseAlphabetical,
    ] {
        println!("{}", diagvrs::demo(&dot, s));
    }
}
