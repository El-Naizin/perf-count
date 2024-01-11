use std::io::{Error, ErrorKind};
use crate::error::PerfCounterBuilderError;
use kperf_rs::error::KperfError;
use crate::PerfCounterError;

impl From<PerfCounterBuilderError> for KperfError {
    fn from(value: PerfCounterBuilderError) -> Self {
        match value {
            PerfCounterBuilderError::UnsupportedCounterType(counter) => {
                KperfError::UnknownError(format!("Unsupported counter: {:?}", counter))
            }
            PerfCounterBuilderError::BuildError(description) => {
                KperfError::PerfCounterBuildError(description)
            }
            PerfCounterBuilderError::IO(err) => KperfError::PermissionDenied,
        }
    }
}

impl From<KperfError> for PerfCounterBuilderError {
    fn from(value: KperfError) -> Self {
        match value {
            KperfError::UnknownError(description) => {
                PerfCounterBuilderError::BuildError(description)
            }
            KperfError::PermissionDenied => {
                PerfCounterBuilderError::BuildError(format!("Permission denied"))
            }
            KperfError::PerfCounterBuildError(description) => {
                PerfCounterBuilderError::BuildError(description)
            }
        }
    }
}

impl From<KperfError> for PerfCounterError {
    fn from(value: KperfError) -> Self {
        match value {
            KperfError::UnknownError(desc) => PerfCounterError::Unknown(desc),
            KperfError::PermissionDenied => PerfCounterError::IO(Error::from(ErrorKind::PermissionDenied)),
            KperfError::PerfCounterBuildError(desc) => PerfCounterError::Unknown(desc),
        }
    }
}

