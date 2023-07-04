extern crate clap;
extern crate rand;

use clap::Parser;

mod diceit;

/// Generates Diceware passwords
#[derive(Parser,Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short,long,default_value_t=5)]
    words: u8,

    #[arg(short,long,default_value="en")]
    list: String,

    #[arg(short,long,default_value_t=false)]
    replace: bool,

    #[arg(short,long,default_value_t=false)]
    verbose: bool,

    #[arg(short,long,default_value=" ")]
    separator: String,
}

fn main() {

    let args = Args::parse();

    diceit::dice_it(
        args.words,
        &args.list,
        args.verbose,
        args.replace,
        &args.separator
        );
}
