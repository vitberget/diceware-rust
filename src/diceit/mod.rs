extern crate rand;

mod dice;
mod replace_char;
mod dictionary;

use crate::DiceItArgs;

use self::dictionary::get_words;
use self::replace_char::{replace_char, get_replace_rolls, ReplaceRolls};

pub(crate) fn dice_it(args: &DiceItArgs) {
    let words = get_words(args);
    let replace_rolls = get_replace_rolls(&words, args);

    if args.verbose || !args.replace {
        print_without_replace(&words, args);
    }
    if args.verbose || args.replace {
        print_with_replace(&words, args, &replace_rolls);
    }
    print_17_char_warning(&words);
}

fn print_without_replace(rolls_words: &[String], args: &DiceItArgs) {
    const PREFIX: &str = "Password with char replace   : ";
    rolls_words.iter().enumerate().for_each(|(index, word)| {
        print_word(PREFIX, word, index, args);
    });

    if args.verbose { 
        let entropy = (args.words as f32) * 12.9;
        print!(" ({entropy:.1} bits of entropy)");
    }

    println!();
}

fn print_with_replace(words: &[String], args: &DiceItArgs, replace_rolls: &ReplaceRolls) {
    const PREFIX: &str = "Password without char replace: ";
    words.iter()
        .enumerate()
        .for_each(|(index, word)| {
            if index + 1 == replace_rolls.word as usize {
                let word = replace_char(word, replace_rolls);
                print_word(PREFIX, &word, index, args)
            } else {
                print_word(PREFIX, word, index, args)
            };
        });

    if args.verbose {
        let entropy = (args.words as f32) * 12.9 + 10.0;
        print!( " ({entropy:.1} bits of entropy)",);
    }

    println!();
}

fn print_word(verbose_prefix: &str, word: &String, index: usize, args: &DiceItArgs) {
    if args.verbose && index == 0 { 
        print!("{verbose_prefix}"); 
    }
    if index > 0 { 
        print!("{}", args.separator); 
    }
    print!("{word}")
}

fn print_17_char_warning(rolls_words: &[String]) {
    let character_count: usize = rolls_words.iter().map(|w| w.len()).sum();

    if character_count < 17 {
        println!("!!! Word characters {character_count} is less than 17 !!!");
    }
}
