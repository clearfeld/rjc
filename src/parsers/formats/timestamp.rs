extern crate chrono;
use chrono::{DateTime, TimeZone, NaiveDateTime, Datelike, Timelike, Local};

use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimestampData {
    // pub meta: Meta,
    pub naive: Naive,
    pub utc: Utc
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Naive {
    pub year: i32,
    pub month: String,
    pub month_num: u32,
    pub day: u32,
    pub weekday: String,
    pub weekday_num: u32,
    pub hour: u32,
    pub hour_24: u32,
    pub minute: u32,
    pub second: u32,
    pub period: String,
    pub day_of_year: u32,
    pub week_of_year: u32,
    pub iso: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Utc {
    pub year: i32,
    pub month: String,
    pub month_num: u32,
    pub day: u32,
    pub weekday: String,
    pub weekday_num: u32,
    pub hour: u32,
    pub hour_24: u32,
    pub minute: u32,
    pub second: u32,
    pub period: String,
    pub utc_offset: String,
    pub day_of_year: u32,
    pub week_of_year: u32,
    pub iso: String,
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

pub fn parse(data: Option<String>) -> TimestampData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

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

    for sl in buffer.lines() {
        let timestamp = sl.trim().parse::<i64>().unwrap();

        let utcdt = NaiveDateTime::from_timestamp_opt(timestamp, (timestamp % 1) as u32 * 1_000_000).unwrap();

        let naivedt: DateTime<Local> = Local.from_utc_datetime(&utcdt);

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
        utc.utc_offset = Local.offset_from_utc_datetime(&utcdt).to_string();
    }

    TimestampData {
        // meta: meta,
        naive: naive,
        utc: utc
    }
}
