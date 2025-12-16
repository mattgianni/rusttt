// struct Game;

use crate::error::AppError;
use crate::cli::Config;

pub fn play(cfg: &Config) -> Result<String, AppError> {
    Ok(format!("processed: {:?}", cfg))
}
