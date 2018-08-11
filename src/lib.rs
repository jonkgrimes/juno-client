extern crate rand;
extern crate tokio;
extern crate hyper;
extern crate uuid;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::thread;
use std::time::Duration;
use uuid::Uuid;
use hyper::{Client, Method, Request, Uri, Body};
use hyper::rt::{self, Future, Stream};
use tokio::runtime::Runtime;

mod models;

use models::MetricData;

pub fn run(telemetry_url: &str) {
  let uuid = Uuid::new_v4();
  let mut runtime = Runtime::new().unwrap(); 
  let uri: Uri = telemetry_url.parse().ok().expect("Couldn't parse telemetry URI");

  // main program loop
  loop {
    // Setup data
    let metric = MetricData::fake(uuid);

    // Setup request
    let client = Client::new();
    let mut req = Request::new(Body::from(serde_json::to_string(&metric).unwrap()));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();

    // Make request
    let post = client.request(req).and_then(|res| {
      println!("POST: {}", res.status());
      res.into_body().concat2()
    })
    // If all good, just tell the user...
    .map(|_| {
        println!("Done.");
    })
    // If there was an error, let the user know...
    .map_err(|err| {
        eprintln!("Error {}", err);
    });

    runtime.spawn(post);

    thread::sleep(Duration::new(3, 0));
  }
  
  // Shut down the tokio thread
  runtime.shutdown_on_idle().wait().unwrap(); 
}