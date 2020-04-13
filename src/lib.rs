pub mod deamon;
pub mod util;

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


#[derive(Debug)]
pub struct Package {
    pub length: usize,
    pub version: u32,
    /// Opcodes:
    /// * 2 - heartbeat package sent by the client
    /// * 3 - popularity
    /// * 5 - notification
    /// * 7 - auth / joining room
    /// * 8 - heartbeat package sent by the server
    pub opcode: u32,
    pub param: u32,
    pub body: Option<String>,
}

impl Package {
    pub fn new() -> Self {
        Package {
            length: 0x0000_0010,
            version: 0x0010_0002,
            opcode: 0x0000_0000,
            param: 0x0000_0001,
            body: None,
        }
    }

    pub fn set_body(&mut self, body: Option<String>) {
        match body {
            Some(body) => {
                self.length = body.as_bytes().len() + 16;
                self.body = Some(body);
            },
            None => {
                self.length = 0x0000_0010;
                self.body = None;
            }
        }
    }

    pub fn join_room(user_id: u32, room_id: u32) -> Self {
        let mut package = Package::new();
        package.opcode = 7;
        let body = format!("{{\"roomid\":{},\"uid\":{}}}", room_id, user_id);
        package.set_body(Some(body));
        package
    }

    pub fn heartbeat() -> Self {
        Package {
            length: 0x0000_0010,
            version: 0x0010_0001,
            opcode: 0x0000_0002,
            param: 0x0000_0001,
            body: None,
        }
    }
}


pub struct LoopCounter {
    count: f64,
    ubound: f64,
}

impl LoopCounter {
    pub fn new(ubound: f64) -> Self {
        LoopCounter { count: ubound - 0.5, ubound }
    }
}

impl Iterator for LoopCounter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 0.5;

        if self.count > self.ubound {
            self.count = 0.5;
        }

        Some(self.count)
    }
}