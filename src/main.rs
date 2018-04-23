extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::cmp;
use std::env;
use std::fs::File;
use std::io::Read;

static DW_EN: &'static str = include_str!("../resources/diceware-en.txt");
static DW_SV: &'static str = include_str!("../resources/diceware-sv.txt");
static REPLACE_CHARS: &'static str = "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789";

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
    println!(" ({} bits of entropy)", (word_count as f32) * 12.9);

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
    println!(" ({} bits of entropy)", (word_count as f32) * 12.9 + 10.0);

    let c = rolls_words.iter()
        .map(|w| w.len())
        .fold(0, |a, s| a+s);
    if c < 17 {
        println!("!!! Word characters {} less than 17 !!!", c);
    }
}


fn replace_char(rolls_words: Vec<&String>, r1: u8, r2: u8, r3: u8, r4: u8) -> String {
    let rc = REPLACE_CHARS.chars().nth(((r4 - 1) * 6 + (r3 - 1)) as usize).unwrap();
    let w = rolls_words[(r1 - 1) as usize];

    let mut w2 = String::new();
    let mut i = 1;
    for c in w.chars() {
        if i == r2 {
            w2.push(rc);
        } else {
            w2.push(c);
        }
        i = i + 1;
    }

    return w2;
}

fn main() {
    let n: u8 = env::args().nth(1).unwrap_or("5".to_string()).parse().expect("could not convert argument to u8");
    let f = env::args().nth(2).unwrap_or("en".to_string());

    dice_it(n, f);
}


#[cfg(test)]
mod tests {
    use get_replace_char;

    #[test]
    fn replace_char1() {
        assert_eq!(get_replace_char(1, 1), '~');
    }

    #[test]
    fn replace_char2() {
        assert_eq!(get_replace_char(6, 3), '}');
    }

    #[test]
    fn replace_char3() {
        assert_eq!(get_replace_char(2, 5), '/');
    }

    #[test]
    fn replace_char4() {
        assert_eq!(get_replace_char(5, 6), '8');
    }
}