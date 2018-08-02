extern crate futures;
#[macro_use]
extern crate hyper;

use std::thread;
use std::time::Duration;
use std::fmt;

struct MetricData {
  cpu: u32,
  memory: u32,
  network_in: u32,
  network_out: u32,
}

impl fmt::Display for MetricData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CPU: {}%, Mem: {}%, Network In: {}%, Network Out: {}%", self.cpu, self.memory, self.network_in, self.network_out)
  }
}

pub fn run(telemetry_url: &str) {
  loop {
    let metric = MetricData {
      cpu: 100,
      memory: 4096,
      network_in: 2134123,
      network_out: 1344592
    };
    println!("{}", metric);
    thread::sleep(Duration::new(10, 0));
  }
}