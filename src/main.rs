extern crate juno_client;

use juno_client::run;

const DEFAULT_URL: &str = "http://localhost:8080/data";

fn main() {
    run(DEFAULT_URL);
}
