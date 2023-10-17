extern crate clap;
extern crate rand;

use clap::Parser;
use diceit::dice_it;

mod diceit;

/// Generates Diceware passwords
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(
    help_template = "{about-section}\nVersion: {version} \nAuthor: {author-with-newline}\n{usage-heading} {usage}\n\n{all-args} {tab}"
)]
struct DiceItArgs {
    #[arg(short, long, default_value_t = 5)]
    words: u8,

    #[arg(short, long, default_value = "en")]
    list: String,

    #[arg(short, long, default_value_t = false)]
    replace: bool,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value = " ")]
    separator: String,
}

fn main() {
    dice_it(&DiceItArgs::parse());
}
