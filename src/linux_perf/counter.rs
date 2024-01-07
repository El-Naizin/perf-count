use crate::error::PerfCounterError;

pub struct LinuxCounter {}

impl LinuxCounter {
    pub fn reset(&mut self) -> Result<(), PerfCounterError> {
        // Ok(self.counter.reset())
        todo!()
    }

    pub fn start(&mut self) -> Result<(), PerfCounterError> {
        // match self.counter.start() {
        //     Ok(_) => Ok(()),
        //     Err(_) => Err(PerfCounterError::Unknown("TODO: error message".to_string())),
        // }
        todo!()
    }

    pub fn stop(&mut self) -> Result<(), PerfCounterError> {
        // match self.counter.stop() {
        //     Ok(_) => Ok(()),
        //     Err(_) => Err(PerfCounterError::Unknown("TODO: error message 2".to_string())),
        // }
        todo!()
    }

    pub fn read(&mut self) -> Result<u64, PerfCounterError> {
        // Ok(self.counter.read())
        todo!()
    }
}
