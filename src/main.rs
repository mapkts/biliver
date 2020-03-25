use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::thread;
use std::time::Duration;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate bilive;
use bilive::deamon;
use bilive::impls::Config;

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

    if matches.is_present("room") {
        config.room_id = matches.value_of("room").unwrap().parse::<u32>().unwrap();
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
            eprintln!("Deamon thread interrupted abnormally, try to restart: {}", e);
        }
        buffer.flush()?;
        thread::sleep(Duration::from_secs(10));
    }

}
