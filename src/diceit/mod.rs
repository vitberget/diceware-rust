extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::cmp;
use std::fs::File;
use std::io::Read;

use self::dice::{dice, dice_strings};
use self::replace_char::replace_char;

mod dice;
mod replace_char;

static DW_EN: &'static str = include_str!("../../resources/diceware-en.txt");
static DW_SV: &'static str = include_str!("../../resources/diceware-sv.txt");

fn read_words(filename: &str, rolls: Vec<String>) -> HashMap<String, String> {
    let mut word_map = HashMap::new();

    for s in rolls.iter() {
        word_map.insert(s.clone(), String::new());
    }

    let mut file_contents = String::new();
    let source = match filename {
        "sv" => DW_SV,
        "en" => DW_EN,
        _ => {
            let mut file = File::open(filename).unwrap();
            file.read_to_string(&mut file_contents).unwrap();
            file_contents.as_str()
        }
    };

    for line in source.to_string().lines() {
        let split: Vec<_> = line.split_whitespace().collect();

        let key = String::from(*split.get(0).unwrap());

        if word_map.contains_key(&key) {
            let val = String::from(*split.get(1).unwrap());
            word_map.insert(key, val);
        }
    }

    return word_map;
}

fn print_rolls(rolls: Vec<String>, r1: u8, r2: u8, r3: u8, r4: u8) {
    print!("Password rolls   :");
    for s in rolls.iter() {
        print!(" {}", s);
    }
    println!();
    println!("Replace rolls    : {}{}{}{}", r1, r2, r3, r4);
}

fn print_option1(rolls_words: Vec<&String>, word_count: u8, verbose: bool, separator: &str) {
    let mut first = true;
    for word in rolls_words {
        if first {
            if verbose {
                print!("Password option 1: ");
            }
            print!("{}", word);
            first = false;
        } else {
            print!("{}{}", separator, word);
        }
    }
    if verbose {
        print!(" ({:.1} bits of entropy)", (word_count as f32) * 12.9);
    }
    println!();
}

fn print_option2(rolls_words: Vec<&String>, word_count: u8, verbose: bool, separator: &str, r1: u8, r2: u8, r3: u8, r4: u8) {
    for (i, word) in rolls_words.clone().iter().enumerate() {
        if i == 0 {
            if verbose {
                print!("Password option 2: ")
            }
        } else {
            print!("{}", separator);
        }

        if i+1 == r1 as usize {
            print!("{}", replace_char(word, r2, r3, r4));
        } else {
            print!("{}", word)
        }
    }
    if verbose {
        print!(" ({:.1} bits of entropy)", (word_count as f32) * 12.9 + 10.0);
    }
    println!();
}

fn print_17_warning(rolls_words: Vec<&String>) {
    let c = rolls_words.iter()
        .map(|w| w.len())
        .fold(0, |a, s| a + s);
    if c < 17 {
        println!("!!! Word characters {} less than 17 !!!", c);
    }
}

pub fn dice_it(word_count: u8, filename: &str, verbose: bool, replace: bool, separator: &str) {
    let rolls = dice_strings(word_count);

    let words = read_words(filename, rolls.clone());
    let rolls_words: Vec<&String> = rolls.iter()
        .map(|r| words.get(r).unwrap())
        .collect();

    let r1 = rand::thread_rng().gen_range(1..=cmp::min(6, word_count)) as u8;
    let r1_word = rolls_words[(r1 - 1) as usize];
    let (_, r1_word_size_upper) = r1_word.chars().size_hint();
    let r2 = rand::thread_rng().gen_range(1..=cmp::min(6, r1_word_size_upper.unwrap())) as u8;
    let r3 = dice();
    let r4 = dice();

    if verbose {
        print_rolls(rolls, r1, r2, r3, r4);
    }
    if verbose || !replace {
        print_option1(rolls_words.clone(), word_count, verbose, separator);
    }
    if verbose || replace {
        print_option2(rolls_words.clone(), word_count, verbose, separator, r1, r2, r3, r4);
    }
    print_17_warning(rolls_words);
}