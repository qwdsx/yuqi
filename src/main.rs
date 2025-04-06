use std::env;
use serde::{Deserialize, Serialize};

use anyhow::Result;

mod tests;
mod parser;

#[derive(Debug, Deserialize, Serialize)]
struct Note {
    __type__: String,
    fields: Vec<String>,
    guid: String,
    note_model_uuid: String,
    tags: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct Notes {
    notes: Vec<Note>
}

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
