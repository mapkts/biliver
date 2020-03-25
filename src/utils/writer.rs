use std::io::prelude::*;
use std::collections::HashSet;
use crate::utils::time::get_date_time;

pub fn write_popularity<B: Write>(popularity: u32, buf: &mut B, threshold: u32) {
    if popularity > threshold {
        let (date, time) = get_date_time();
        let line = format!("人气,{},{},{}\r\n", date, time, popularity);

        if let Err(e) = buf.write(&line.as_bytes()) {
            eprintln!("ERROR: cannot write data to log file: {}", e);
        }

        if let Err(e) = buf.flush() {
            eprintln!("ERROR: cannot flush buffer contents: {}", e);
        }
    }
}

pub fn write_barrage<B: Write>(uid: u64, uname: &str, msg: &str, buf: &mut B, excludes: &HashSet<u32>, no_print: bool) {
    if !excludes.contains(&(uid as u32)) {
        let (date, time) = get_date_time();
        let line = format!("弹幕,{},{},{},\"{}\",\"{}\"\r\n", date, time, uid, uname, msg);

        if !no_print {
            let padding = " ".repeat( inc_if_neg(30 - get_visual_width(uname), 2) );
            println!("[{}]      {}{}{}", time, uname, padding, msg);
        }

        if let Err(e) = buf.write(&line.as_bytes()) {
            eprintln!("ERROR: cannot write data to log file: {}", e);
        }
    }
}

pub fn write_gift<B: Write>(uid: u64, uname: &str, gift_name: &str, num: u64, coin_type: &str, total_coin: u64, buf: &mut B, no_silver: bool) {
    if (no_silver == true && coin_type != "silver") || no_silver == false {
        let (date, time) = get_date_time();
        let line = format!("礼物,{},{},{},\"{}\",\"{}\",{},{},{}\r\n", date, time, uid, uname, gift_name, num, total_coin, coin_type);

        if let Err(e) = buf.write(&line.as_bytes()) {
            eprintln!("ERROR: cannot write data to log file: {}", e);
        }
    }
}

fn get_visual_width(str: &str) -> isize {
    let mut width = 0;
    for char in str.chars() {
        if char.is_ascii() {
            width += 1;
        } else {
            width += 2;
        }
    }
    width
}

fn inc_if_neg(mut num: isize, inc: usize) -> usize {
    while num <= 0 {
        num += inc as isize
    }
    num as usize
}