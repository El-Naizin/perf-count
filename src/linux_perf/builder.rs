use crate::error::PerfCounterBuilderError;
use crate::event::Event;
use crate::linux_perf::linux::{HardwareEventType, PerfCounterBuilderLinux, SoftwareEventType};
use crate::linux_perf::LinuxCounter;

pub struct LinuxCounterBuilder {
    linux_builder: PerfCounterBuilderLinux,
}

impl LinuxCounterBuilder {
    pub fn new() -> Self {
        LinuxCounterBuilder {
            linux_builder: PerfCounterBuilderLinux::default(),
        }
    }

    pub fn set_target_event(mut self, event: Event) -> Result<Self, PerfCounterBuilderError> {
        let hardware_event = <HardwareEventType>::try_from(event.clone());
        match hardware_event {
            Ok(evt) => {
                Ok( Self {
                    linux_builder: self.linux_builder.set_hardware_event(evt),
                })
            }
            Err(_) => match <SoftwareEventType>::try_from(event) {
                Ok(evt) => {
                    Ok( Self {
                        linux_builder: self.linux_builder.set_software_event(evt),
                    })
                }
                Err(e) => Err(e),
            },
        }
    }

    pub fn build(self) -> Result<LinuxCounter, PerfCounterBuilderError> {
        let counter = self.linux_builder.finish()?;
        Ok(LinuxCounter::from(counter))
    }
}
