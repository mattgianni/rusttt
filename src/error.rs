use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Msg(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Msg(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for AppError {}
