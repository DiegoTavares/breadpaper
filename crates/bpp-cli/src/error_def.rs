use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum BppCliError {
    FailedToAddNote,
    FailedToRmNote,
    InvalidParameters(String),
    FailedToConnect(String),
}

impl fmt::Display for BppCliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BppCliError::FailedToAddNote => f.write_str("Failed to Add Note"),
            BppCliError::FailedToRmNote => f.write_str("Failed to Remove Note"),
            BppCliError::InvalidParameters(msg) => {
                f.write_str(format!("Invalid Parameters: {msg}").as_str())
            }
            BppCliError::FailedToConnect(msg) => f.write_str(msg),
        }
    }
}

impl Error for BppCliError {}
