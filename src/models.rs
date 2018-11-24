extern crate serde_json;
extern crate rand;
extern crate uuid;

use std::fmt;
use uuid::Uuid;
use rand::distributions::{Range, Sample};
use serde_derive::*;

#[derive(Serialize)]
pub struct AgentRegistration {
  hostname: String,
  ip: String,
}

impl AgentRegistration {
  pub fn new(hostname: String, ip: String) -> AgentRegistration {
    AgentRegistration { hostname: hostname, ip: ip }
  }
}

#[derive(Serialize)]
pub struct MetricData {
  agent_id: String,
  cpu: u32,
  memory: u32,
  network_in: u32,
  network_out: u32,
}

impl MetricData {
  pub fn fake(uuid: Uuid) -> MetricData {
    let mut cpu_range = Range::new(45, 100);
    let mut memory_range = Range::new(2000, 8000);
    let mut network_range = Range::new(10_000, 50_000);
    let mut rng = rand::thread_rng();

    MetricData {
      agent_id: format!("{}", uuid),
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