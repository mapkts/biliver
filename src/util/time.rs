use chrono::prelude::*;

pub fn get_date_time() -> (String, String) {
  let local: DateTime<Local> = Local::now();
  let date = local.date().format("%F");
  let time = local.time().format("%T");

  (date.to_string(), time.to_string())
}