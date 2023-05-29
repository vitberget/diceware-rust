extern crate clap;
extern crate rand;

use clap::{Arg, App};

mod diceit;

fn main() {
    let version =  env!("CARGO_PKG_VERSION");

    let matches = App::new("Diceware")
        .version(version)
        .author("Kenneth Hedman <github@vitberget.se>")
        .about("Generates Diceware passwords")
        .arg(Arg::with_name("words")
             .short('w')
             .long("words")
             .value_name("NUMBER")
             .help("Number of words in password")
             .default_value("5")
             .takes_value(true))
        .arg(Arg::with_name("list")
             .short('l')
             .long("list")
             .help("Which Diceware list to use")
             .value_name("LIST")
             .help("en, sv or filename")
             .default_value("en")
             .takes_value(true))
        .arg(Arg::with_name("replace")
             .short('r')
             .long("replace")
             .help("Replace one random character"))
        .arg(Arg::with_name("verbose")
             .short('v')
             .long("verbose")
             .help("Verbose response, showing dice rolls and entropy"))
        .arg(Arg::with_name("separator")
             .short('s')
             .long("separator")
             .help("String separating words")
             .default_value(" "))
        .get_matches();

    diceit::dice_it(
        matches.value_of("words").unwrap_or("5").parse().expect("Not a number"), 
        matches.value_of("list").unwrap_or("en"), 
        matches.is_present("verbose"), 
        matches.is_present("replace"), 
        matches.value_of("separator").unwrap()
        );
}
