extern crate rand;

use rand::Rng;

pub fn dice() -> u8 {
    return rand::thread_rng().gen_range(1..=6) as u8;
}

fn five_dice() -> String {
    return format!("{}{}{}{}{}", dice(), dice(), dice(), dice(), dice());
}

pub fn dice_strings(n: u8) -> Vec<String> {
    let mut v = Vec::new();

    for _ in 0..n {
        v.push(five_dice());
    }

    return v;
}