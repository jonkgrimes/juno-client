extern crate rand;

use std::fmt;
use rand::distributions::{Range, Sample};

pub struct MetricData {
  cpu: u32,
  memory: u32,
  network_in: u32,
  network_out: u32,
}

impl MetricData {
  pub fn fake() -> MetricData {
    let mut cpu_range = Range::new(45, 100);
    let mut memory_range = Range::new(2000, 5000);
    let mut network_range = Range::new(10_000, 50_000);
    let mut rng = rand::thread_rng();

    MetricData {
      cpu: cpu_range.sample(&mut rng),
      memory: memory_range.sample(&mut rng),
      network_in: network_range.sample(&mut rng),
      network_out: network_range.sample(&mut rng)
    }
  }
}

impl fmt::Display for MetricData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CPU: {}%, Mem: {} MB, Network In: {} bytes, Network Out: {} bytes", self.cpu, self.memory, self.network_in, self.network_out)
  }
}