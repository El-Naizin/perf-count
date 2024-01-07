use crate::error::PerfCounterBuilderError;
use kperf_rs::error::KperfError;

impl From<PerfCounterBuilderError> for KperfError {
    fn from(value: PerfCounterBuilderError) -> Self {
        match value {
            PerfCounterBuilderError::UnsupportedCounterType(counter) => {
                KperfError::UnknownError(format!("Unsupported counter: {:?}", counter))
            }
            PerfCounterBuilderError::BuildError(description) => {
                KperfError::PerfCounterBuildError(description)
            }
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
