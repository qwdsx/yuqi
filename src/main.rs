use std::env;

mod parser;
use parser::*;

mod tests;

fn main() {
    let _ = match env::args().nth(1) {
        Some(s) => println!("{}", add_tones(&s)),
        None => println!("Enter a valid string.") 
    };
}
