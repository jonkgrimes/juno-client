use std::thread;
use std::time::Duration;

pub fn run() {
  loop {
    println!("Hello World!");
    thread::sleep(Duration::new(60, 0));
  }
}