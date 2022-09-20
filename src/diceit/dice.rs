extern crate rand;

use rand::Rng;

pub fn dice() -> u8 {
    return rand::thread_rng().gen_range(1..=6) as u8
}

fn five_dice() -> String {
    return format!("{}{}{}{}{}", dice(), dice(), dice(), dice(), dice());
}

pub fn dice_strings(n: u8) -> Vec<String> {
    let mut v = Vec::new();

    let mut i = 0;
    while i < n {
        v.push(five_dice());
        i = i + 1;
    }

    return v;
}