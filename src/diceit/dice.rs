use rand::random_range;

pub(crate) fn dice() -> u8 {
    random_range(1..=6) as u8
}

pub(crate) fn dice_strings(n: u8) -> Vec<String> {
    (0..n)
        .map(|_| five_dice())
        .collect()
}

fn five_dice() -> String {
    format!("{}{}{}{}{}", dice(), dice(), dice(), dice(), dice())
}
