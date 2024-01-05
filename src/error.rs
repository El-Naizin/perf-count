use std::{error, fmt, io};
use std::error::Error;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum PerfCounterBuilderError {
    UnsupportedCounterType,
}

/// Possible Errors when using a valid performance counter
#[derive(Debug)]
pub enum PerfCounterError {
    IO(io::Error),
}

impl fmt::Display for PerfCounterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PerfCounterError::IO(e) => {write!(f, "IO Error: {}", e)},
        }
    }
}

impl error::Error for PerfCounterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PerfCounterError::IO(err) => { Some(err)}
        }
    }
}
