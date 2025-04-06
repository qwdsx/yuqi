use std::env;

use anyhow::Result;

mod tests;
mod parser;

fn main() -> Result<()> {
    match env::args().nth(1) {
        Some(s) => {
            match parser::add_tones(&s) {
                Ok(ok) => println!("{:?}", ok),
                Err(_) => println!("Error while adding tones. Check your input."),
            }
        },
        None => println!("Enter a string as the first argument.")
    }

    Ok(())
}