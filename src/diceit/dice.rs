extern crate rand;

use rand::Rng;

pub(crate) fn dice() -> u8 {
    rand::thread_rng().gen_range(1..=6) as u8
}

pub(crate) fn dice_strings(n: u8) -> Vec<String> {
    (0..n).into_iter().map(|_| five_dice()).collect()
}

fn five_dice() -> String {
    format!("{}{}{}{}{}", dice(), dice(), dice(), dice(), dice())
}
