use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ttt", version, about)]
pub struct Cli {
    /// Verbose logging (repeatable: -v, -vv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub verbose: u8,
}

impl From<Cli> for Config {
    fn from(c: Cli) -> Self {
        Self {
            verbose: c.verbose,
        }
    }
}
