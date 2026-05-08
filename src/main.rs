use clap::Parser as _;

use args::DiceItArgs;
use diceit::dice_it;

mod diceit;
mod args;

fn main() {
    let args = DiceItArgs::parse();

    match args.completion {
        Some(shell) => DiceItArgs::completion(shell),
        None => dice_it(&args)
    }
}
