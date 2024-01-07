use crate::error::PerfCounterError;
use crate::PerfCounter;
use kperf_rs;
use kperf_rs::PerfCounterBuilder;

pub struct Counter {
    counter: kperf_rs::PerfCounter,
}

impl From<kperf_rs::PerfCounter> for Counter {
    fn from(value: kperf_rs::PerfCounter) -> Self {
        Self {
            counter: value,
        }
    }
}

impl PerfCounter for Counter {
    fn reset(&self) -> Result<(), PerfCounterError> {
        todo!()
    }

    fn start(&self) -> Result<(), PerfCounterError> {
        todo!()
    }

    fn stop(&self) -> Result<(), PerfCounterError> {
        todo!()
    }

    fn read(&mut self) -> Result<u64, PerfCounterError> {
        todo!()
    }
}
