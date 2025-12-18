pub mod app;
pub mod cli;
pub mod error;
pub mod ttt;

use cli::{Cli, Config};
use error::AppError;
use log::trace;

pub fn run(cli: Cli) -> Result<(), AppError> {
    trace!("run({:?}) called.", &cli);
    let cfg: Config = cli.into();
    app::run(cfg)
}
