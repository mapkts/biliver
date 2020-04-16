use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    thread,
    time::Duration,
};

#[macro_use]
extern crate clap;
use clap::{App, Arg};
use reqwest;
use serde_json::Value;

use biliver::deamon;
use biliver::Config;

fn main() -> std::io::Result<()> {
    let mut config = Config::from_toml("./conf.toml");

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("room")
                .short("r")
                .long("room")
                .takes_value(true)
                .value_name("ROOM_ID")
                .help("Listens to the provided room"),
        )
        .arg(
            Arg::with_name("no-print")
                .long("no-print")
                .help("Do not print danmu to the console"),
        )
        .get_matches();

    let room: u32 = if matches.is_present("room") {
        matches.value_of("room").unwrap().parse::<u32>().unwrap()
    } else {
        config.room_id
    };

    match reqwest::blocking::get(&format!(
        "https://api.live.bilibili.com/room/v1/Room/room_init?id={}",
        room
    ))
    .ok()
    {
        Some(resp) => {
            let body = resp.text().unwrap();
            let json: Value = serde_json::from_str(&body).unwrap();
            config.room_id = json["data"]["room_id"].as_u64().unwrap() as u32;
        }
        None => config.room_id = room,
    }

    if matches.is_present("no-print") {
        config.room_id = matches.value_of("room").unwrap().parse::<u32>().unwrap();
    }

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}-log.csv", config.room_id))
        .unwrap();

    let mut buffer = BufWriter::with_capacity(1024, file);

    loop {
        if let Err(e) = deamon::main_loop(config.clone(), &mut buffer) {
            buffer.flush()?;
            eprintln!("Deamon thread interrupted abnormally: {}", e);
            eprintln!("Trying to restart deamon thread...");
        }
        thread::sleep(Duration::from_secs(10));
    }
}
