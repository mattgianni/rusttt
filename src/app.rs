use crate::{cli::Config, error::AppError};

pub fn run(cfg: Config) -> Result<(), AppError> {
    // Do any top-level setup here (logging, validation, wiring infra)
    // then call into domain/app logic.

    let _ = crate::ttt::game::play(&cfg)?;

    Ok(())
}
