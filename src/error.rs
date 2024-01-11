use crate::event::Event;
use std::fmt::{Debug, Formatter};
use std::{error, fmt, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PerfCounterBuilderError {
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
    #[error("Unsupported counter type: {0:?}")]
    UnsupportedCounterType(Event),
    #[error("Unspecified build error: {0}")]
    BuildError(String),
}

/// Possible Errors when using a valid performance counter
#[derive(Error, Debug)]
pub enum PerfCounterError {
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
    #[error("Unspecified error: {0}")]
    Unknown(String),
}
