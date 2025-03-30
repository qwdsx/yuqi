use std::env;

mod parser;
use parser::*;

mod tests;

fn main() {
    let word = env::args().nth(1).unwrap();
    println!("{}", add_tones(&word));
}