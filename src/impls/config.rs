use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u32,
    pub room_id: u32,
    pub user_id: u32,
    pub no_print: bool,
    pub log_interval: u32,
    pub log_threshold: u32,
    pub ignores: HashSet<u32>,
    pub no_silver: bool,
}

impl Config {
    pub fn from_toml(path: &str) -> Self {
        let mut file = File::open(path).expect("file not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("cannot read config file");

        let config: Config = toml::from_str(&contents).unwrap();

        Config {
            host: config.host,
            port: config.port,
            room_id: config.room_id,
            user_id: config.user_id,
            no_print: config.no_print,
            log_interval: config.log_interval,
            log_threshold: config.log_threshold,
            ignores: config.ignores,
            no_silver: config.no_silver,
        }
    }
}