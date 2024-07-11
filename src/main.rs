extern crate clap;
extern crate rand;

use clap::Parser as _;

use args::DiceItArgs;
use diceit::dice_it;

mod diceit;
mod args;

fn main() {
    dice_it(&DiceItArgs::parse());
}
