use std::fs::OpenOptions;

#[macro_use]
extern crate clap;
extern crate bilive;

use clap::{App, Arg};
use bilive::impls::Config;
use bilive::deamon;

fn main() {
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
            .help("Listens to a specific room")
      )
      .arg(
        Arg::with_name("no-print")
            .long("no-print")
            .help("Do not print danmu to the console")
      )
      .get_matches();

  if matches.is_present("room") {
    config.room_id = matches.value_of("room").unwrap().parse::<u32>().unwrap();
  }

  if matches.is_present("no-print") {
    config.room_id = matches.value_of("room").unwrap().parse::<u32>().unwrap();
  }


  let mut log_file = OpenOptions::new()
                  .append(true)
                  .create(true)
                  .open(format!("{}-log.csv", config.room_id))
                  .unwrap();

  deamon::main_loop(config, &mut log_file);
}