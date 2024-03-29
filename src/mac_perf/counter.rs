use crate::error::PerfCounterError;
use kperf_rs;

pub struct MacCounter {
    counter: kperf_rs::PerfCounter,
}

impl MacCounter {
    pub fn reset(&mut self) -> Result<(), PerfCounterError> {
        Ok(self.counter.reset()?)
    }

    pub fn start(&mut self) -> Result<(), PerfCounterError> {
        Ok(self.counter.start()?)
    }

    pub fn stop(&mut self) -> Result<(), PerfCounterError> {
        Ok(self.counter.stop()?)
    }

    pub fn read(&mut self) -> Result<u64, PerfCounterError> {
        Ok(self.counter.read()?)
    }
}

impl From<kperf_rs::PerfCounter> for MacCounter {
    fn from(value: kperf_rs::PerfCounter) -> Self {
        Self { counter: value }
    }
}
