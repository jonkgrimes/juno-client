extern crate juno_client;
extern crate toml;
extern crate uuid;

use std::fs;
use juno_client::run;
use toml::Value;
use uuid::Uuid;

const DEFAULT_HOST: &str = "http://localhost:8080";
const CONFIG_FILE: &str =  "config.toml";

fn main() {
    // read config file
    let config: Value = fs::read_to_string(CONFIG_FILE)
        .expect("Couldn't read the config file")
        .parse()
        .expect("Couldn't parse the config file");

    // parse the config file
    let uuid = Uuid::parse_str(config["uuid"].as_str().unwrap()).unwrap();

    run(Some(uuid), DEFAULT_HOST);
}
