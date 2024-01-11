use std::thread;
use std::time::Duration;
use perf_count::{event::Event, PerfCounter, PerfCounterBuilder};

fn long_function() {
    thread::sleep(Duration::from_secs(2));
}

fn main() {
    let mut counter = PerfCounterBuilder::new()
        .set_target_event(Event::CPUCycles)
        .expect("Invalid event type")
        .build()
        .expect("Couldn't build counter");

    counter.start().unwrap();
    long_function();
    counter.stop().unwrap();
    let total_cycles = counter.read().unwrap();
    println!("Total cpu cycles: {}", total_cycles);
}
