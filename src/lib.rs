pub mod error;
pub mod event;

pub use crate::error::PerfCounterBuilderError;
use crate::event::Event;
use cfg_if::cfg_if;
pub use error::PerfCounterError;

cfg_if! {
    if #[cfg(target_os="macos")] {
        mod mac_perf;
        pub type PerfCounter = mac_perf::MacCounter;
        pub type PerfCounterBuilder = mac_perf::MacCounterBuilder;
    } else if #[cfg(target_os="linux")] {
        mod linux_perf;
        pub type PerfCounter = linux_perf::LinuxCounter;
        pub type PerfCounterBuilder = linux_perf::LinuxCounterBuilder;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_counter() {
        let builder = PerfCounterBuilder::new();
        let x = builder.set_target_event(Event::CPUCycles).unwrap();
        let mut counter = x.build().unwrap();
        counter.start().expect("Couldn't start counter");
        counter.stop().expect("Couldn't stop counter");
        let result = counter.read().expect("Couldn't read counter");
        assert_ne!(result, 0, "Cycle count shouldn't be zero");
    }

    #[test]
    fn test_unimplemented_counter() {
        let builder = PerfCounterBuilder::new();
        let x = builder.set_target_event(Event::CacheReferences).unwrap();
        let mut counter = x.build().unwrap();
        counter.start().expect("Couldn't start counter");
        counter.stop().expect("Couldn't stop counter");
        let result = counter.read().expect("Couldn't read counter");
        assert_ne!(result, 0, "Cycle count shouldn't be zero");
    }
}
