use crate::event::Event;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::{error, fmt, io};

#[derive(Debug)]
pub enum PerfCounterBuilderError {
    UnsupportedCounterType(Event),
    BuildError(String),
}

/// Possible Errors when using a valid performance counter
#[derive(Debug)]
pub enum PerfCounterError {
    IO(io::Error),
    Unknown(String),
}

impl fmt::Display for PerfCounterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PerfCounterError::IO(e) => {
                write!(f, "IO Error: {}", e)
            }
            PerfCounterError::Unknown(description) => {
                write!(f, "Unknown Error: {}", description)
            }
        }
    }
}

impl error::Error for PerfCounterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PerfCounterError::IO(err) => Some(err),
            PerfCounterError::Unknown(_) => None,
        }
    }
}
