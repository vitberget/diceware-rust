use std::collections::HashMap;
use std::fs;

use crate::DiceItArgs;

use super::dice::dice_strings;

static DICTIONARY_ENGLISH: &str = include_str!("../../resources/diceware-en.txt");
static DICTIONARY_SWEDISH: &str = include_str!("../../resources/diceware-sv.txt");

pub(crate) fn get_words(args: &DiceItArgs) -> Vec<String> {
    let rolls = dice_strings(args.words);

    if args.verbose {
        print!("Password rolls:");
        rolls.iter().for_each(|roll| print!(" {roll}"));
        println!();
    }

    let word_map = read_words(&args.list, &rolls);

    rolls.into_iter()
        .map(|roll| word_map.get(&roll).unwrap().to_owned())
        .collect()
}

fn read_words(filename: &str, rolls: &[String]) -> HashMap<String, String> {
    get_dictionary(filename)
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|splitted_line| (String::from(*splitted_line.first().unwrap()), splitted_line))
        .filter(|(dice_string,_)| rolls.contains(dice_string))
        .map(|(dice_string,splitted_line)| {
            let word = String::from(*splitted_line.get(1).unwrap());
            (dice_string, word)
        })
        .collect()
}

fn get_dictionary(filename: &str) -> String {
    match filename {
        "sv" => DICTIONARY_SWEDISH.to_string(),
        "en" => DICTIONARY_ENGLISH.to_string(),

        filename => fs::read_to_string(filename).unwrap()
    }
}
