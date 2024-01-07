use crate::error::PerfCounterBuilderError;
use crate::event::Event;
use crate::{PerfCounter, PerfCounterBuilder};
use kperf_rs::event::Event as Kevent;
use crate::mac_perf::MacCounter;

pub struct MacCounterBuilder {
    mac_builder: kperf_rs::PerfCounterBuilder,
}

impl MacCounterBuilder {
    pub fn new() -> Self {
        Self {
            mac_builder: kperf_rs::PerfCounterBuilder::new(),
        }
    }

    pub fn set_target_event(mut self, event: Event) -> Result<Self, PerfCounterBuilderError> {
        let kperf_event: Kevent = event.try_into()?;
        self.mac_builder = self.mac_builder.track_event(kperf_event);
        Ok(self)
    }

    pub fn build(mut self) -> Result<MacCounter, PerfCounterBuilderError> {
        let counter = self.mac_builder.build_counter()?;
        let perf_counter: MacCounter = counter.into();
        Ok(perf_counter)
    }
}
