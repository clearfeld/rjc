use std::io::{self, BufRead};

use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TimedatectlData {
    // meta: Meta,
    resources: Resources,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resources {
    local_time: String,
    universal_time: String,
    rtc_time: String,
    time_zone: String,
    ntp_enabled: bool,
    system_clock_synchronized: bool,
    rtc_in_local_tz: bool,
    epoc_utc: i64,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    // let mut meta = Meta {};

    let mut r = Resources {
        local_time: String::new(),
        universal_time: String::new(),
        rtc_time: String::new(),
        time_zone: String::new(),
        ntp_enabled: false,
        system_clock_synchronized: false,
        rtc_in_local_tz: false,
        epoc_utc: 0
    };

    for line in handle.lines() {
        let mut sl = String::from(line.unwrap().trim());
        let mut field= String::from(sl.split_once(":").unwrap().0);
        field.push_str(": ");
        sl = String::from(sl.trim_start_matches(&field));

        match field.as_str() {
            "Local time: " => {r.local_time = sl}
            "Universal time: " => {
                r.universal_time = sl.clone();
                sl = String::from(sl.trim_end_matches(" UTC"));
                let datetime = Utc.datetime_from_str(sl.split_once(" ").unwrap().1, "%Y-%m-%d %H:%M:%S").unwrap();
                r.epoc_utc = datetime.timestamp();
            }
            "RTC time: " => {r.rtc_time = sl}
            "Time zone: " => {r.time_zone = sl}
            "System clock synchronized: " => {r.system_clock_synchronized = sl == "yes"}
            "NTP service: " => {r.ntp_enabled = sl == "yes"}
            "RTC in local TZ: " => {r.rtc_in_local_tz = sl == "yes"}
            _ => {}
        }
    }

    r_io_utils::print_output::<TimedatectlData>(
        &TimedatectlData {
            // meta: meta,
            resources: r,
        },
        output_type,
    );
}
