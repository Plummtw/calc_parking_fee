#![allow(dead_code)]
extern crate chrono;
use chrono::{Datelike, Timelike, Local};
use chrono::naive::NaiveTime;

mod singledayfee;
mod fee;

fn main() {
    let now = Local::now();
    println!("date is {:04}:{:02}:{:02}, time is {:02}:{:02}:{:02} weekday is {}",
        now.year(), now.month(),  now.day(),
        now.hour(), now.minute(), now.second(), now.weekday());

    let time_only1 = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S").unwrap();
    println!("{}", time_only1);
    let time_only2 = NaiveTime::parse_from_str("23:58:25", "%H:%M:%S").unwrap();
    println!("{}", time_only2);
    let duration = time_only2 - time_only1;
    println!("{}", duration.num_minutes());

    let minutes : Vec<i32>= vec![0, 10, 17, 30, 50, 60, 70, 90, 120, 150, 180, 900];
    for minute in minutes {
        println!("{} : {} ", minute, fee::fee(minute));
    }
}

