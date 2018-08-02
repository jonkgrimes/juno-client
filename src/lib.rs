extern crate rand;

use std::thread;
use std::time::Duration;
use std::fmt;

mod models;

use models::MetricData;

pub fn run(telemetry_url: &str) {
  loop {
    let metric = MetricData::fake();
    println!("{}", metric);
    thread::sleep(Duration::new(10, 0));
  }
}