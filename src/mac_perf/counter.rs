use crate::error::PerfCounterError;
use crate::PerfCounter;

pub struct Counter {
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
