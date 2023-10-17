use crate::DiceItArgs;
use rand::Rng;
use std::cmp::min;
use super::dice::dice;

static REPLACE_CHARS: &str = "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789";

pub(crate) struct ReplaceRolls {
    pub(crate) word: u8,
    pub(crate) char: u8,
    pub(crate) new_char_1: u8,
    pub(crate) new_char_2: u8,
}

pub(crate) fn replace_char(the_word: &str, replace_rolls: &ReplaceRolls) -> String {
    the_word
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i + 1 == replace_rolls.char as usize {
                REPLACE_CHARS
                    .chars()
                    .nth(((replace_rolls.new_char_1 - 1) * 6 + (replace_rolls.new_char_2 - 1)) as usize)
                    .unwrap()
            } else {
                c
            }
        })
        .collect()
}

pub(crate) fn get_replace_rolls(words: &[String], args: &DiceItArgs) -> ReplaceRolls {
    let replace_word_roll = rand::thread_rng().gen_range(1..=min(6, args.words));
    let r1_word = &words[(replace_word_roll - 1) as usize];
    let (_, r1_word_size_upper) = r1_word.chars().size_hint();
    let replace_char_roll = rand::thread_rng().gen_range(1..=min(6, r1_word_size_upper.unwrap())) as u8;

    let rolls = ReplaceRolls { 
        word: replace_word_roll, 
        char: replace_char_roll, 
        new_char_1: dice(), 
        new_char_2: dice() 
    };
    if args.verbose {
        println!("Replace rolls    : {}{}{}{}",
                rolls.word,
                rolls.char,
                rolls.new_char_1,
                rolls.new_char_2);
    };
    rolls
}
