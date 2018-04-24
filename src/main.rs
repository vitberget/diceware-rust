extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::cmp;
use std::env;
use std::fs::File;
use std::io::Read;

use dice::{dice, dice_strings};
use replace_char::{replace_char};
mod dice;
mod replace_char;

static DW_EN: &'static str = include_str!("../resources/diceware-en.txt");
static DW_SV: &'static str = include_str!("../resources/diceware-sv.txt");

fn read_words(filename: String, rolls: Vec<String>) -> HashMap<String, String> {
    let mut word_map = HashMap::new();

    for s in rolls.iter() {
        word_map.insert(s.clone(), String::new());
    }

    let mut file_contents = String::new();
    let source = match filename.as_str() {
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

fn dice_it(word_count: u8, filename: String) {
    let rolls = dice_strings(word_count);

    let words = read_words(filename, rolls.clone());
    let rolls_words: Vec<&String> = rolls.iter().map(|r| words.get(r).unwrap()).collect();

    let r1 = rand::thread_rng().gen_range(1, cmp::min(7, word_count + 1));
    let r1_word = rolls_words[(r1 - 1) as usize];
    let (_, r1_word_size_upper) = r1_word.chars().size_hint();
    let r2 = (rand::thread_rng().gen_range(1, cmp::min(7, r1_word_size_upper.unwrap() + 1))) as u8;
    let r3 = dice();
    let r4 = dice();

    let pw2 = replace_char(rolls_words.clone(), r1, r2, r3, r4);

    print!("Password rolls   :");
    for s in rolls.iter() {
        print!(" {}", s);
    }
    println!();
    println!("Replace rolls    : {}{}{}{}", r1, r2, r3, r4);

    let mut first = true;
    for w in rolls_words.clone() {
        if first {
            print!("Password option 1: {}", w);
        } else {
            print!("_{}", w);
        }
        first = false;
    }
    println!(" ({:.1} bits of entropy)", (word_count as f32) * 12.9);

    first = true;
    let mut i = 1;
    for w in rolls_words.clone() {
        if first {
            print!("Password option 2: ")
        } else {
            print!("_");
        }

        if i == r1 {
            print!("{}", pw2)
        } else {
            print!("{}", w)
        }

        first = false;
        i = i + 1;
    }
    println!(" ({:.1} bits of entropy)", (word_count as f32) * 12.9 + 10.0);

    let c = rolls_words.iter()
        .map(|w| w.len())
        .fold(0, |a, s| a+s);
    if c < 17 {
        println!("!!! Word characters {} less than 17 !!!", c);
    }
}

fn main() {
    let n: u8 = env::args().nth(1).unwrap_or("5".to_string()).parse().expect("could not convert argument to u8");
    let f = env::args().nth(2).unwrap_or("en".to_string());

    dice_it(n, f);
}