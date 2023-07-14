use std::io::{self, BufRead};
extern crate chrono;
use chrono::{DateTime, TimeZone, NaiveDateTime, Datelike, Timelike, Local};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TimestampData {
    // meta: Meta,
    naive: Naive,
    utc: Utc
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Naive {
    year: i32,
    month: String,
    month_num: u32,
    day: u32,
    weekday: String,
    weekday_num: u32,
    hour: u32,
    hour_24: u32,
    minute: u32,
    second: u32,
    period: String,
    day_of_year: u32,
    week_of_year: u32,
    iso: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Utc {
    year: i32,
    month: String,
    month_num: u32,
    day: u32,
    weekday: String,
    weekday_num: u32,
    hour: u32,
    hour_24: u32,
    minute: u32,
    second: u32,
    period: String,
    utc_offset: String,
    day_of_year: u32,
    week_of_year: u32,
    iso: String,
}

fn get_month(month_num: u32) -> String {
    let mut month_name = String::from("");
    match month_num {
        1 => {month_name = String::from("Jan")}
        2 => {month_name = String::from("Feb")}
        3 => {month_name = String::from("Mar")}
        4 => {month_name = String::from("Apr")}
        5 => {month_name = String::from("May")}
        6 => {month_name = String::from("Jun")}
        7 => {month_name = String::from("Jul")}
        8 => {month_name = String::from("Aug")}
        9 => {month_name = String::from("Sepr")}
        10 => {month_name = String::from("Oct")}
        11 => {month_name = String::from("Nov")}
        12 => {month_name = String::from("Dec")}
        _ => {}
    }
    month_name
}

fn get_day_of_week(num_from_monday: u32) -> String {
    let mut weekday = String::from("");
    match num_from_monday {
        1 => {weekday = String::from("Mon")}
        2 => {weekday = String::from("Tue")}
        3 => {weekday = String::from("Wed")}
        4 => {weekday = String::from("Thu")}
        5 => {weekday = String::from("Fri")}
        6 => {weekday = String::from("Sat")}
        7 => {weekday = String::from("Sun")}
        _ => {}
    }
    weekday
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    // let mut meta = Meta {};
    let mut naive = Naive {
        year: 0,
        month: String::new(),
        month_num: 0,
        day: 0,
        weekday: String::new(),
        weekday_num: 0,
        hour: 0,
        hour_24: 0,
        minute: 0,
        second: 0,
        period: String::new(),
        day_of_year: 0,
        week_of_year: 0,
        iso: String::new(),
    };
    let mut utc = Utc {
        year: 0,
        month: String::new(),
        month_num: 0,
        day: 0,
        weekday: String::new(),
        weekday_num: 0,
        hour: 0,
        hour_24: 0,
        minute: 0,
        second: 0,
        period: String::new(),
        day_of_year: 0,
        week_of_year: 0,
        iso: String::new(),
        utc_offset: String::new()
    };

    for line in handle.lines() {
        let sl = line.unwrap();
        let timestamp = sl.parse::<i64>().unwrap();

        let naivedt = NaiveDateTime::from_timestamp(timestamp, (timestamp % 1) as u32 * 1_000_000);

        let utcdt: DateTime<Local> = Local.from_utc_datetime(&naivedt);

        naive.year = naivedt.year();
        utc.year = utcdt.year();
        naive.month = get_month(naivedt.month());
        utc.month = get_month(utcdt.month());
        naive.month_num = naivedt.month();
        utc.month_num = utcdt.month();

        naive.weekday = get_day_of_week(naivedt.weekday().number_from_monday());
        utc.weekday = get_day_of_week(utcdt.weekday().number_from_monday());
        
        naive.weekday_num = naivedt.weekday().number_from_monday();
        naive.day = naivedt.day();
        naive.hour = naivedt.hour12().1;
        if naivedt.hour12().0 {
            naive.period = String::from("PM");
        }
        else {
            naive.period = String::from("AM");
        }
        naive.hour_24 = naivedt.hour();
        naive.minute = naivedt.minute();
        naive.second = naivedt.second();
        naive.iso = naivedt.to_string().replace(" ", "T");
        naive.day_of_year = naivedt.ordinal();
        naive.week_of_year = naivedt.ordinal()/7;

        utc.weekday_num = utcdt.weekday().number_from_monday();
        utc.day = utcdt.day();
        utc.hour = utcdt.hour12().1;
        if utcdt.hour12().0 {
            utc.period = String::from("PM");
        }
        else {
            utc.period = String::from("AM");
        }
        utc.hour_24 = utcdt.hour();
        utc.minute = utcdt.minute();
        utc.second = utcdt.second();
        utc.iso = utcdt.to_string().replace(" ", "T");
        utc.day_of_year = utcdt.ordinal();
        utc.week_of_year = utcdt.ordinal()/7;
        utc.utc_offset = Local.offset_from_utc_datetime(&naivedt).to_string();
    }

    r_io_utils::print_output::<TimestampData>(
        &TimestampData {
            // meta: meta,
            naive: naive,
            utc: utc
        },
        output_type,
    );
}
