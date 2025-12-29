use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rusttt", version, about = "RusTTT, a tic-tac-toe game.")]
pub struct Cli {
    /// Debug logging (-vv for trace level logging)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub verbose: u8,
}

impl From<Cli> for Config {
    fn from(c: Cli) -> Self {
        Self { verbose: c.verbose }
    }
}
