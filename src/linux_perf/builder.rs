use crate::error::PerfCounterBuilderError;
use crate::event::Event;
use crate::linux_perf::LinuxCounter;

pub struct LinuxCounterBuilder {
    // mac_builder: kperf_rs::PerfCounterBuilder,
}

impl LinuxCounterBuilder {
    pub fn new() -> Self {
        todo!();
        // Self {
        //     // mac_builder: kperf_rs::PerfCounterBuilder::new(),
        // }
    }

    pub fn set_target_event(mut self, event: Event) -> Result<Self, PerfCounterBuilderError> {
        // let kperf_event: Kevent = event.try_into()?;
        // self.mac_builder = self.mac_builder.track_event(kperf_event);
        // Ok(self)
        todo!()
    }

    pub fn build(self) -> Result<LinuxCounter, PerfCounterBuilderError> {
        // let counter = self.mac_builder.build_counter()?;
        // let perf_counter: LinuxCounterBuilder = counter.into();
        // Ok(perf_counter)
        todo!()
    }
}
