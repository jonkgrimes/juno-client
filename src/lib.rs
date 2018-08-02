extern crate rand;
extern crate tokio;
extern crate hyper;

use std::thread;
use std::time::Duration;
use hyper::{Client, Method, Request, Uri, Body};
use hyper::rt::{self, Future, Stream};
use tokio::runtime::Runtime;

mod models;

use models::MetricData;

pub fn run(telemetry_url: &str) {
  let mut runtime = Runtime::new().unwrap(); 
  let uri: Uri = telemetry_url.parse().ok().expect("Couldn't parse telemetry URI");

  loop {
    // Setup data
    let metric = MetricData::fake();

    // Setup request
    let client = Client::new();
    let mut req = Request::new(Body::from(format!("{}", metric)));
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

    thread::sleep(Duration::new(10, 0));
  }
  
  runtime.shutdown_on_idle().wait().unwrap(); 
}