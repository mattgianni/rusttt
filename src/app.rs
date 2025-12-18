use crate::{cli::Config, error::AppError};
use log::trace;

pub fn run(cfg: Config) -> Result<(), AppError> {
    // Do any top-level setup here (logging, validation, wiring infra)
    // then call into domain/app logic.
    trace!("run({:?}) called.", &cfg);

    let _ = crate::ttt::game::play(&cfg)?;

    Ok(())
}
