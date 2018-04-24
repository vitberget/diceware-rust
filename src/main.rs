use std::env;

mod diceit;

fn main() {
    let n: u8 = env::args().nth(1).unwrap_or("5".to_string()).parse().expect("could not convert argument to u8");
    let f = env::args().nth(2).unwrap_or("en".to_string());

    diceit::dice_it(n, f);
}