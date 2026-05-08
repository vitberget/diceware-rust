use clap::{CommandFactory as _, Parser};
use clap_complete::{Generator, Shell, generate};

/// Generates Diceware passwords
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(
    help_template = "{about-section}\nVersion: {version} \nAuthor: {author-with-newline}\n{usage-heading} {usage}\n\n{all-args} {tab}"
)]
pub struct DiceItArgs {
    #[arg(short, long, default_value_t = 5)]
    /// How many words to roll
    pub words: u8,

    #[arg(short, long, default_value = "en")]
    /// List to be used, sv, en or file
    pub list: String,

    #[arg(short, long, default_value_t = false)]
    /// Replace a random letter in a randow word with a random special character
    pub replace: bool,

    #[arg(short, long, default_value_t = false)]
    /// Be verbose
    pub verbose: bool,

    #[arg(short, long, default_value = " ")]
    /// What to use as a separator between words
    pub separator: String,

    #[arg(short, long)]
    /// Print shell completion
    pub completion: Option<Shell>
}

impl DiceItArgs {
    pub fn completion<G: Generator>(generator: G) {
        let mut cmd = Self::command();
        let name = cmd.get_name().to_string();
        generate(generator, &mut cmd, name, &mut std::io::stdout());
    }
}
