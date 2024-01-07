use crate::error::PerfCounterBuilderError;
use crate::event::Event;
use crate::{PerfCounter, PerfCounterBuilder};
use kperf_rs::event::Event as Kevent;
use crate::perf::counter::Counter;

pub struct CounterBuilder {
    mac_builder: kperf_rs::PerfCounterBuilder,
}

impl CounterBuilder {
    pub fn new() -> Self {
        Self {
            mac_builder: kperf_rs::PerfCounterBuilder::new(),
        }
    }
}

impl PerfCounterBuilder for CounterBuilder {
    fn set_target_event(mut self: Box<Self>, event: Event) -> Result<Box<dyn PerfCounterBuilder>, PerfCounterBuilderError> {
        let kperf_event: Kevent = event.try_into()?;
        self.mac_builder = self.mac_builder.track_event(kperf_event);
        Ok(self)
    }

    fn build(self: Box<Self>) -> Result<Box<dyn PerfCounter>, PerfCounterBuilderError> {
        let counter = self.mac_builder.build_counter()?;
        let perf_counter: Counter = counter.into();
        Ok(Box::new(perf_counter))
    }
}
