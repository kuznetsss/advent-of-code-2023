use std::{fmt::Display, io};

#[derive(Debug)]
pub enum AocError {
    IoError(io::Error, String),
    DigitNotFound(String),
    ErrorDownloadingInput(String),
    CantParseConfig(String, String),
}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AocError::*;
        match self {
            IoError(err, path) => {
                write!(f, "IO error: {err} for \"{path}\"")
            }
            DigitNotFound(line) => {
                write!(f, "Can't find a digit in line: {line}")
            }
            ErrorDownloadingInput(err) => {
                write!(f, "Error downloading input: {err}")
            }
            CantParseConfig(err, file) => {
                write!(f, "Error parsing file {file}: {err}")
            }
        }
    }
}
