use std::fs;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let path = &args[1];

    let contents = fs::read_to_string(path).expect("Should have been able to read file");

    println!("{contents}")
}
