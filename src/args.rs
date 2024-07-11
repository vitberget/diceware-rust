use clap::{command, Parser};

/// Generates Diceware passwords
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(
    help_template = "{about-section}\nVersion: {version} \nAuthor: {author-with-newline}\n{usage-heading} {usage}\n\n{all-args} {tab}"
)]
pub struct DiceItArgs {
    #[arg(short, long, default_value_t = 5)]
    pub words: u8,

    #[arg(short, long, default_value = "en")]
    pub list: String,

    #[arg(short, long, default_value_t = false)]
    pub replace: bool,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[arg(short, long, default_value = " ")]
    pub separator: String,
}
