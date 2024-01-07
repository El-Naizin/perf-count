pub mod error;
pub mod event;

use cfg_if::cfg_if;
pub use error::PerfCounterError;
pub use crate::error::PerfCounterBuilderError;
use crate::event::Event;

cfg_if! {
    if #[cfg(target_os="macos")] {
        mod mac_perf;
        pub type PerfCounter = mac_perf::MacCounter;
        pub type PerfCounterBuilder = mac_perf::MacCounterBuilder;
    } else if #[cfg(target_os="linux")] {
        mod linux_perf;
        pub type PerfCounter = linux_perf::Counter;
        pub type PerfCounterBuilder = linux_perf::CounterBuilder;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_build_cycle_counter() {
        let builder = PerfCounterBuilder::new();
        let x = builder.set_target_event(Event::CPUCycles).unwrap();
        let _counter = x.build().unwrap();
        // let counter = builder.set_target_event(Event::CPUCycles).unwrap().build().unwrap();
    }
}