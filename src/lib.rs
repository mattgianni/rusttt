pub mod app;
pub mod cli;
pub mod ttt;
pub mod error;

use cli::{Cli, Config};
use error::AppError;

pub fn run(cli: Cli) -> Result<(), AppError> {
    let cfg: Config = cli.into();
    app::run(cfg)
}
