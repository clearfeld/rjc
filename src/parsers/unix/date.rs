use std::ops::Add;

use chrono::{DateTime, Duration, Datelike, Timelike};
use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateData {
    // pub meta: Meta,
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
    pub timezone: String,
    pub utc_offset: String,
    pub day_of_year: u32,
    pub week_of_year: u32,
    pub iso: String,
    pub epoch: i64,
    pub epoch_utc: i64
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Meta {
// TODO:
// }

pub fn parse(data: Option<String>) -> DateData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};

    let mut r = DateData {
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
        timezone: String::new(),
        utc_offset: String::new(),
        day_of_year: 0,
        week_of_year: 0,
        iso: String::new(),
        epoch: 0,
        epoch_utc: 0
    };

    for line in buffer.lines() {
        let sl = String::from(line.trim());
        let mut date_split = sl.split_whitespace();
        let timezone = date_split.next_back().unwrap();
        let period = date_split.next_back().unwrap();
        let mut date_str = String::from(sl.trim_end_matches(timezone).trim());
        date_str = String::from(date_str.trim_end_matches(period).trim());

        let datetime_str = String::from(date_str) + " " + chrono::Local::now().offset().to_string().as_str();
        let mut date = DateTime::parse_from_str(datetime_str.as_str(), "%a %d %b %Y %H:%M:%S %:z").unwrap();

        if period == "PM" {
            date = date.add(Duration::hours(12));
        }
        r.year = date.year();
        r.month_num = date.month();
        match date.month() {
            1 => {r.month = String::from("Jan")}
            2 => {r.month = String::from("Feb")}
            3 => {r.month = String::from("Mar")}
            4 => {r.month = String::from("Apr")}
            5 => {r.month = String::from("May")}
            6 => {r.month = String::from("Jun")}
            7 => {r.month = String::from("Jul")}
            8 => {r.month = String::from("Aug")}
            9 => {r.month = String::from("Sep")}
            10 => {r.month = String::from("Oct")}
            11 => {r.month = String::from("Nov")}
            12 => {r.month = String::from("Dec")}
            _ => {}
        }
        r.day = date.day();
        match date.weekday() {
            chrono::Weekday::Mon => {
                r.weekday = String::from("Mon");
                r.weekday_num = 1;
            },
            chrono::Weekday::Tue => {
                r.weekday = String::from("Tue");
                r.weekday_num = 2;
            },
            chrono::Weekday::Wed => {
                r.weekday = String::from("Wed");
                r.weekday_num = 3;
            },
            chrono::Weekday::Thu => {
                r.weekday = String::from("Thu");
                r.weekday_num = 4;
            },
            chrono::Weekday::Fri => {
                r.weekday = String::from("Fri");
                r.weekday_num = 5;
            },
            chrono::Weekday::Sat => {
                r.weekday = String::from("Sat");
                r.weekday_num = 6;
            },
            chrono::Weekday::Sun => {
                r.weekday = String::from("Sun");
                r.weekday_num = 7;
            },
        }
        r.hour = date.hour12().1;
        r.hour_24 = date.hour();
        r.minute = date.minute();
        r.second = date.second();
        r.period = String::from(period);
        r.timezone = String::from(timezone);
        r.week_of_year = date.iso_week().week();
        r.utc_offset = date.timezone().to_string();
        r.week_of_year = date.iso_week().week();
        r.epoch = date.timestamp();
        let mut utc_date = date.naive_utc();
        utc_date = utc_date.with_year(r.year).unwrap();
        utc_date = utc_date.with_day(r.day).unwrap();
        utc_date = utc_date.with_month(r.month_num).unwrap();
        utc_date = utc_date.with_hour(r.hour_24).unwrap();
        r.epoch_utc = utc_date.timestamp();

        r.day_of_year = date.ordinal();

        r.iso = format!("{}-{}-{}T{}:{}:{}{}", r.year, r.month_num, r.day, r.hour_24, r.minute, r.second, r.utc_offset);
    }

    r
}
