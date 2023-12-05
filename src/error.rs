use std::{fmt::Display, io};

#[derive(Debug)]
pub enum AocError {
    IoError(io::Error, String),
}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AocError::*;
        match self {
            IoError(err, path) => {
                write!(f, "IO error: {err} for \"{path}\"")
            },
        }
    }
}
