use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum BppConfigError {
    FailedToLoadConfigOptions(String),
}

impl fmt::Display for BppConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BppConfigError::FailedToLoadConfigOptions(msg) => {
                f.write_str(format!("Invalid config options: {msg}").as_str())
            }
        }
    }
}

impl Error for BppConfigError {}
