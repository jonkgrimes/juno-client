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
use hyper::header::HeaderValue;
use tokio::runtime::Runtime;

mod models;

use models::MetricData;

pub fn run(config_uuid: Option<Uuid>, telemetry_host: &str) {
  // hyper run time setup
  let mut runtime = Runtime::new().unwrap(); 

  // determine if agent has registered previously
  let uuid = match config_uuid {
    Some(id) => id,
    None => {
      register(&mut runtime, telemetry_host)
    }
  };
  let telemetry_url = format!("{}/data", telemetry_host);
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

fn register(runtime: &mut Runtime, telemetry_host: &str) -> Uuid {
    let client = Client::new();
    let telemetry_url = format!("{}/register", telemetry_host);
    let uri: Uri = telemetry_url.parse().ok().expect("Couldn't parse telemetry URI");
    let body = r#"{"hostname":"orion","ip":"10.10.40.3"}"#;
    let mut req = Request::new(Body::from(body));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(hyper::header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let register_req = client.request(req).and_then(|response| {
      response.into_body().concat2()
    }).and_then(|body| {
      let s = std::str::from_utf8(&body).expect("Expected UTF-8 ins response");
      println!("s = {}", s);
      Ok(())
    }).map_err(|_| {
      eprintln!("An error occurred attempting to register the agent");
    });

    runtime.spawn(register_req);

    Uuid::new_v4()
}