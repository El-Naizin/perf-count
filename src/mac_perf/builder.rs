use crate::error::PerfCounterBuilderError;
use crate::event::Event;
use crate::{PerfCounter, PerfCounterBuilder};

pub struct CounterBuilder {
}

impl CounterBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl PerfCounterBuilder for CounterBuilder {
    fn set_target_event(&mut self, event: Event) -> Result<Box<dyn PerfCounterBuilder>, PerfCounterBuilderError> {
        todo!()
    }

    fn build(self) -> Result<Box<dyn PerfCounter>, PerfCounterBuilderError> {
        todo!()
    }
}
