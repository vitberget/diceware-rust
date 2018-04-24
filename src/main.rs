extern crate clap;

use clap::{Arg, App};

mod diceit;

fn main() {
    let matches = App::new("Diceware")
        .version("0.2")
        .author("Kenneth Hedman <github@vitberget.se>")
        .about("Generates Diceware passwords")
        .arg(Arg::with_name("words")
            .short("w")
            .long("words")
            .value_name("NUMBER")
            .help("Number of words in password")
            .default_value("5")
            .takes_value(true))
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("Which Diceware list to use")
            .value_name("LIST")
            .help("en, sv or filename")
            .default_value("en")
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Verbose response, showing dice rolls and entropy"))
        .get_matches();
    let wordcount: u8 = matches.value_of("words").unwrap_or("5").parse().expect("Not a number");
    let list = matches.value_of("list").unwrap_or("en");
    let verbose  = matches.is_present("verbose");

    diceit::dice_it(wordcount, list, verbose);
}