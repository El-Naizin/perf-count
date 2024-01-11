use crate::error::PerfCounterError;
use crate::linux_perf::linux::PerfCounter;

pub struct LinuxCounter {
    counter: PerfCounter,
}

impl LinuxCounter {
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

impl From<PerfCounter> for LinuxCounter {
    fn from(value: PerfCounter) -> Self {
        Self {
            counter: value,
        }
    }
}