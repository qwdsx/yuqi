use serde::{Serialize, Deserialize};
use serde_json;

use regex::Regex;
use regex_split::*;

// use colored::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Vowel {
    pub vowel: char,
    pub tones: Vec::<char>
}

pub fn replace_v_with_umlaut(str: &str) -> String {
    str.replace("v", "ü")
}

pub fn remove_numbers(str: &str) -> String {
    let re = Regex::new(r"[0-9]").unwrap();
    re.replace_all(str, "").to_string()
}

pub fn check_vowel_priority(str: &str) -> char {
    let vowel_priority = ['a', 'o', 'e', 'i', 'u', 'ü'];
    let mut priority_vowel: char = vowel_priority[0];

    for v in vowel_priority {
        if str.contains(v) {
            priority_vowel = v;
            break;
        }
    }

    priority_vowel
}

pub fn get_tone(str: &str) -> u8 {
    let last_char = str.chars().last().unwrap();

    if last_char.to_string().parse::<u8>().is_ok()
    && last_char.to_string().parse::<u8>().unwrap() < 5
        {
        last_char.to_string().parse::<u8>().unwrap()
    } else {
        5
    }
}

pub fn add_tones(str: &str) -> String {
    let json_file = include_str!("../vowel.json");
    let vowels: Vec::<Vowel> = serde_json::from_str(&json_file).unwrap();

    let replace_v_with_umlaut = replace_v_with_umlaut(str);
    let words = divide_into_words(&replace_v_with_umlaut);

    let mut final_sentence = String::new();

    for (i, w) in words.iter().enumerate() {
        let mut final_word = String::new();
        let syllables = get_syllables(&w);
    
        for s in syllables {
            let priority_vowel = check_vowel_priority(&s);
            let correct_vowel = vowels.iter().find(|v| v.vowel == priority_vowel).unwrap();
            let tone = get_tone(&s);
            
            if tone > 0 && tone < 5 {
                let with_tone = s.replace(
                    priority_vowel,
                    &correct_vowel.tones[(tone - 1) as usize]
                    .to_string());
                final_word.push_str(&with_tone);
            } else {
                final_word.push_str(&s);
            }
        }

        let separator =
            if i == words.len() - 1 {
                "".to_string()
            } else {
                " ".to_string()
            };

        final_sentence.push_str(
            &format!("{}{}", &final_word, separator)
        );
    }

    remove_numbers(&final_sentence)
}

pub fn get_syllables(str: &str) -> Vec<String> {
    let re = Regex::new("[0-9]").unwrap();
    let syllables = re
        .split_inclusive(str)
        .collect::<Vec<&str>>()
        .iter()
        .map(|f| f.to_string())
        .filter(|p| !p.is_empty())
        .collect::<Vec<String>>();
    // syllables.truncate(syllables.len() - 1);
    syllables
}

pub fn divide_into_words(str: &str) -> Vec<String> {
    str
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
}

// fn colorize_tone(str: &str, tone: u8) -> ColoredString {
//     let colored = match tone {
//         1 => str.red(),
//         2 => str.yellow(),
//         3 => str.green(),
//         4 => str.blue(),
//         _ => str.normal()
//     };
//
//     colored
// }
