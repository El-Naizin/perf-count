pub mod error;
pub mod event;

#[cfg_attr(target_os = "macos", path = "./mac_perf/mod.rs")]
#[cfg_attr(target_os = "linux", path = "./linux_perf/mod.rs")]
mod perf;

use error::PerfCounterError;
use crate::error::PerfCounterBuilderError;
use crate::event::Event;

//TODO: implement this function
pub fn create_counter_builder() -> Box<dyn PerfCounterBuilder> {
    Box::new(perf::CounterBuilder::new())
}

//TODO: demo different execution on mac and linux
pub trait PerfCounterBuilder {
    fn set_target_event(self: Box<Self>, event: Event) -> Result<Box<dyn PerfCounterBuilder>, PerfCounterBuilderError>;

    fn build(self: Box<Self>) -> Result<Box<dyn PerfCounter>, PerfCounterBuilderError>;
}

/// Abstract trait to control performance counters.
pub trait PerfCounter {
    /// Reset performance counter.
    fn reset(&self) -> Result<(), PerfCounterError>;

    /// Start measuring.
    fn start(&self) -> Result<(), PerfCounterError>;

    /// Stop measuring.
    fn stop(&self) -> Result<(), PerfCounterError>;

    /// Read the counter value.
    fn read(&mut self) -> Result<u64, PerfCounterError>;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_build_cycle_counter() {
        let builder = create_counter_builder();
        let x = builder.set_target_event(Event::CPUCycles).unwrap();
        let counter = x.build().unwrap();
        // let counter = builder.set_target_event(Event::CPUCycles).unwrap().build().unwrap();
    }
}