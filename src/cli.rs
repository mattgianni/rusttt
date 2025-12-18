use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rusttt", version, about = "RusTTT, a tic-tac-toe game.")]
pub struct Cli {
    /// Debug logging (-vv for trace level logging)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Computer plays itself
    #[arg(short, long)]
    pub solo: bool,
    /// Number of games to play (if solo)
    #[arg(short, long, default_value_t = 1)]
    pub number: u128,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub verbose: u8,
    pub solo: bool,
    pub number: u128,
}

impl From<Cli> for Config {
    fn from(c: Cli) -> Self {
        Self {
            verbose: c.verbose,
            solo: c.solo,
            number: c.number,
        }
    }
}
