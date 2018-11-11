#[macro_use]
extern crate serde_derive;
extern crate juno_client;
extern crate toml;
extern crate uuid;

use std::fs;
use juno_client::run;
use toml::Value;
use uuid::Uuid;

const DEFAULT_HOST: &str = "http://localhost:8080";
const CONFIG_FILE: &str =  "config.toml";

#[derive(Debug, Deserialize)]
struct Config {
    uuid: Option<String>,
}

fn main() {
    // read  and parse config file
    println!("Read and parse config.toml");
    let config_file = fs::read_to_string(CONFIG_FILE).expect("Couldn't read config file");
    let config: Config = toml::from_str(config_file.as_str()).expect("Couldn't parse config file");

    println!("config = {:?}", config);
    // extract the uuid
    if let Some(uuid_str) = config.uuid {
        let uuid = Uuid::parse_str(uuid_str.as_str()).unwrap();
        run(Some(uuid), DEFAULT_HOST);
    } else {
        run(None, DEFAULT_HOST);
    }
}
