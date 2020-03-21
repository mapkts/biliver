use std::io::prelude::*;
use std::collections::HashSet;
use crate::utils::time::get_date_time;
use encoding::{all::GB18030, EncoderTrap, Encoding};

pub fn write_popularity<B: Write>(popularity: u32, buf: &mut B, threshold: u32) {
    if popularity > threshold {
        let (date, time) = get_date_time();
        let line = format!("Popularity,{},{},{}\r\n", date, time, popularity);
        let line_u8: Vec<u8> = GB18030.encode(&line, EncoderTrap::Replace).unwrap();

        if let Err(e) = buf.write(&line_u8) {
            eprintln!("ERROR: cannot write data to log file: {}", e);
        }
    }
}

pub fn write_barrage<B: Write>(uid: u64, uname: &str, msg: &str, buf: &mut B, excludes: &HashSet<u32>, no_print: bool) {
    if !excludes.contains(&(uid as u32)) {
        let (date, time) = get_date_time();
        let line = format!("Barrage,{},{},{},\"{}\",\"{}\"\r\n", date, time, uid, uname, msg);
        let line_u8: Vec<u8> = GB18030.encode(&line, EncoderTrap::Replace).unwrap();

        if !no_print {
            let padding = " ".repeat( 30 - get_visual_width(uname) );
            println!("[{}]      {}{}{}", time, uname, padding, msg);
        }

        if let Err(e) = buf.write(&line_u8) {
            eprintln!("ERROR: cannot write data to log file: {}", e);
        }
    }
}

pub fn write_gift<B: Write>(uid: u64, uname: &str, gift_name: &str, num: u64, coin_type: &str, total_coin: u64, buf: &mut B, no_silver: bool) {
    if (no_silver == true && coin_type != "silver") || no_silver == false {
        let (date, time) = get_date_time();
        let line = format!("Gift,{},{},{},\"{}\",\"{}\",{},{},{}\r\n", date, time, uid, uname, gift_name, num, total_coin, coin_type);
        let line_u8: Vec<u8> = GB18030.encode(&line, EncoderTrap::Replace).unwrap();

        if let Err(e) = buf.write(&line_u8) {
            eprintln!("ERROR: cannot write data to log file: {}", e);
        }
    }
}

fn get_visual_width(str: &str) -> usize {
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