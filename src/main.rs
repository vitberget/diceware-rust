extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

static DW_EN: &'static str = include_str!("../resources/diceware-en.txt");
static DW_SV: &'static str = include_str!("../resources/diceware-sv.txt");

fn dice() -> u8 {
    return rand::thread_rng().gen_range(1, 7);
}

fn five_dice() -> String {
    return format!("{}{}{}{}{}", dice(), dice(), dice(), dice(), dice());
}

fn dice_strings(n: u8) -> Vec<String> {
    let mut v = Vec::new();

    let mut i = 0;
    while i < n {
        v.push(five_dice());
        i = i + 1;
    }

    return v;
}

fn read_words(filename: String, rolls: Vec<String>) -> HashMap<String, String> {
    let mut word_map = HashMap::new();

    for s in rolls.iter() {
        word_map.insert(s.clone(), String::from(""));
    }

    let source = if filename == "en" { DW_EN } else { DW_SV };
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

    print!("Dice rolls:");
    for s in rolls.iter() {
        print!(" {}", s);
    }
    println!();

    let words = read_words(filename, rolls.clone());

    let rolls_words: Vec<&String> = rolls.iter().map(|r| words.get(r).unwrap()).collect();

    let mut first = true;
    for w in rolls_words {
        if first {
            print!("Password  : {}", w);
        } else {
            print!("_{}", w);
        }
        first = false;
    }
    println!();
}

fn main() {
    let n: u8 = env::args().nth(1).unwrap_or("5".to_string()).parse().expect("could not convert argument to u8");
    let f = env::args().nth(2).unwrap_or("en".to_string());

    dice_it(n, f);
}
