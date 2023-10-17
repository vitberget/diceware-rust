extern crate rand;

mod dice;
mod replace_char;

use rand::Rng;
use std::{cmp, collections::HashMap, fs::File, io::Read};

use self::{
    dice::{dice, dice_strings},
    replace_char::replace_char,
};

static DICTIONARY_ENGLISH: &'static str = include_str!("../../resources/diceware-en.txt");
static DICTIONARY_SWEDISH: &'static str = include_str!("../../resources/diceware-sv.txt");

fn read_words(filename: &str, rolls: &Vec<String>) -> HashMap<String, String> {
    let mut word_map = HashMap::new();

    rolls.iter().for_each(|s| {
        word_map.insert(s.clone(), String::new());
    });

    let source = match filename {
        "sv" => DICTIONARY_SWEDISH.to_string(),
        "en" => DICTIONARY_ENGLISH.to_string(),
        _ => {
            let mut file_contents = String::new();
            let mut file = File::open(filename).unwrap();
            file.read_to_string(&mut file_contents).unwrap();
            file_contents
        }
    };

    source.lines().for_each(|line| {
        let split: Vec<_> = line.split_whitespace().collect();
        let key = String::from(*split.get(0).unwrap());

        if word_map.contains_key(&key) {
            let val = String::from(*split.get(1).unwrap());
            word_map.insert(key, val);
        }
    });

    return word_map;
}

fn print_rolls(rolls: Vec<String>, r1: u8, r2: u8, r3: u8, r4: u8) {
    print!("Password rolls   :");
    rolls.iter().for_each(|roll| print!(" {roll}"));
    println!();
    println!("Replace rolls    : {r1}{r2}{r3}{r4}");
}

fn print_without_replace(rolls_words: &Vec<&String>, word_count: u8, verbose: bool, separator: &str) {
    rolls_words.into_iter().enumerate().for_each(|(index, word)| {
        if index == 0 {
            if verbose {
                print!("Password with char replace   : ");
            }
            print!("{word}");
        } else {
            print!("{separator}{word}");
        }
    });

    if verbose {
        print!(" ({:.1} bits of entropy)", (word_count as f32) * 12.9);
    }

    println!();
}

fn print_with_replace(
    rolls_words: &Vec<&String>,
    word_count: u8,
    verbose: bool,
    separator: &str,
    replace_word_roll: u8,
    replace_char_roll: u8,
    new_char_roll_1: u8,
    new_char_roll_2: u8,
) {
    rolls_words
        .clone()
        .iter()
        .enumerate()
        .for_each(|(index, word)| {
            if index == 0 {
                if verbose {
                    print!("Password without char replace: ");
                }
            } else {
                print!("{separator}");
            }

            if index + 1 == replace_word_roll as usize {
                print!("{}", replace_char(word, replace_char_roll, new_char_roll_1, new_char_roll_2));
            } else {
                print!("{word}");
            }
        });
    if verbose {
        print!(
            " ({:.1} bits of entropy)",
            (word_count as f32) * 12.9 + 10.0
        );
    }
    println!();
}

fn print_17_char_warning(rolls_words: &Vec<&String>) {
    let char_count: usize = rolls_words.iter().map(|w| w.len()).sum();

    if char_count < 17 {
        println!("!!! Word characters {char_count} less than 17 !!!");
    }
}

pub(crate) fn dice_it(
    word_count: u8,
    filename: &String,
    verbose: bool,
    replace: bool,
    separator: &String,
) {
    let rolls = dice_strings(word_count);

    let word_map = read_words(filename, &rolls);
    let words: Vec<&String> = rolls.iter().map(|r| word_map.get(r).unwrap()).collect();

    let replace_word_roll = rand::thread_rng().gen_range(1..=cmp::min(6, word_count)) as u8;
    let r1_word = words[(replace_word_roll - 1) as usize];
    let (_, r1_word_size_upper) = r1_word.chars().size_hint();
    let replace_char_roll = rand::thread_rng().gen_range(1..=cmp::min(6, r1_word_size_upper.unwrap())) as u8;
    let new_char_roll_1 = dice();
    let new_char_roll_2 = dice();

    if verbose {
        print_rolls(rolls, replace_word_roll, replace_char_roll, new_char_roll_1, new_char_roll_2);
    }
    if verbose || !replace {
        print_without_replace(&words, word_count, verbose, separator);
    }
    if verbose || replace {
        print_with_replace(
            &words,
            word_count,
            verbose,
            separator,
            replace_word_roll,
            replace_char_roll,
            new_char_roll_1,
            new_char_roll_2,
        );
    }
    print_17_char_warning(&words);
}
