use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::DiceItArgs;

use super::dice::dice_strings;

static DICTIONARY_ENGLISH: &str = include_str!("../../resources/diceware-en.txt");
static DICTIONARY_SWEDISH: &str = include_str!("../../resources/diceware-sv.txt");

pub(crate) fn read_words(filename: &str, rolls: &[String]) -> HashMap<String, String> {
    let mut word_map = HashMap::new();
    rolls.iter().for_each(|s| { word_map.insert(s.clone(), String::new()); });

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
        let key = String::from(*split.first().unwrap());

        if word_map.contains_key(&key) {
            let val = String::from(*split.get(1).unwrap());
            word_map.insert(key, val);
        }
    });

    word_map
}

pub(crate) fn get_words(args: &DiceItArgs) -> Vec<String> {
    let rolls = dice_strings(args.words);

    if args.verbose {
        print!("Password rolls   :");
        rolls.iter().for_each(|roll| print!(" {roll}"));
        println!();
    }

    let word_map = read_words(&args.list, &rolls);

    rolls.into_iter()
        .map(|roll| word_map.get(&roll).unwrap().to_owned())
        .collect()
}
