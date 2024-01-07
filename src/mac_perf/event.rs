use crate::error::PerfCounterBuilderError;
use crate::event::Event;
use kperf_rs::event::Event as Kevent;

impl TryFrom<Event> for Kevent {
    type Error = PerfCounterBuilderError;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value {
            Event::CPUCycles => Ok(Kevent::Cycles),
            Event::Instructions => Ok(Kevent::Instructions),
            Event::BranchInstructions => Ok(Kevent::Branches),
            Event::BranchMisses => Ok(Kevent::BranchMisses),
            Event::CacheReferences | Event::CacheMisses => {
                Err(PerfCounterBuilderError::UnsupportedCounterType(value))
            }
        }
    }
}
