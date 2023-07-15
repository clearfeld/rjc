use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimedatectlData {
    // pub meta: Meta,
    pub resources: Resources,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub local_time: String,
    pub universal_time: String,
    pub rtc_time: String,
    pub time_zone: String,
    pub ntp_enabled: bool,
    pub system_clock_synchronized: bool,
    pub rtc_in_local_tz: bool,
    pub epoc_utc: i64,
}

pub fn parse(data: Option<String>) -> TimedatectlData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

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

    for line in buffer.lines() {
        let mut sl = String::from(line.trim());
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

    TimedatectlData {
        // meta: meta,
        resources: r,
    }
}
