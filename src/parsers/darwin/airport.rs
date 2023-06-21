use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AirportData {
    #[serde(rename = "agrCtlRSSI")]
    agr_ctl_rssi: i32,
    #[serde(rename = "agrExtRSSI")]
    agr_ext_rssi: i32,
    #[serde(rename = "agrCtlNoise")]
    agr_ctl_noise: i32,
    #[serde(rename = "agrExtNoise")]
    agr_ext_noise: i32,
    state: String,
    #[serde(rename = "op mode")]
    op_mode: String,
    #[serde(rename = "lastTxRate")]
    last_tx_rate: i32,
    #[serde(rename = "MaxRate")]
    max_rate: i32,
    #[serde(rename = "lastAssocStatus")]
    last_assoc_status: i32,
    #[serde(rename = "802.11 auth")]
    e_auth: String,
    #[serde(rename = "link auth")]
    link_auth: String,
    #[serde(rename = "BSSID")]
    bssid: String,
    #[serde(rename = "SSID")]
    ssid: String,
    #[serde(rename = "MCS")]
    mcs: i32,
    #[serde(rename = "guardInterval")]
    guard_interval: i32,
    #[serde(rename = "NSS")]
    nss: i32,
    channel: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut ad = AirportData {
        agr_ctl_rssi: 0,
        agr_ext_rssi: 0,
        agr_ctl_noise: 0,
        agr_ext_noise: 0,
        state: String::new(),
        op_mode: String::new(),
        last_tx_rate: 0,
        max_rate: 0,
        last_assoc_status: 0,
        e_auth: String::new(),
        link_auth: String::new(),
        bssid: String::new(),
        ssid: String::new(),
        mcs: 0,
        guard_interval: 0,
        nss: 0,
        channel: String::new(),
    };

    for line in handle.lines() {
        let sl = line.unwrap();

        if let Some((left, right)) = sl.split_once(":") {
            // println!("{:?} {:?}", left.trim(), right.trim());

            match left.trim() {
                "agrCtlRSSI" => {
                    ad.agr_ctl_rssi = right.trim().parse::<i32>().unwrap();
                }

                "agrExtRSSI" => {
                    ad.agr_ext_rssi = right.trim().parse::<i32>().unwrap();
                }

                "agrCtlNoise" => {
                    ad.agr_ctl_noise = right.trim().parse::<i32>().unwrap();
                }

                "agrExtNoise" => {
                    ad.agr_ext_noise = right.trim().parse::<i32>().unwrap();
                }

                "state" => {
                    ad.state = right.trim().to_string();
                }

                "op mode" => {
                    ad.op_mode = right.trim().to_string();
                }

                "lastTxRate" => {
                    ad.last_tx_rate = right.trim().parse::<i32>().unwrap();
                }

                "maxRate" => {
                    ad.max_rate = right.trim().parse::<i32>().unwrap();
                }

                "lastAssocStatus" => {
                    ad.last_assoc_status = right.trim().parse::<i32>().unwrap();
                }

                "802.11 auth" => {
                    ad.e_auth = right.trim().to_string();
                }

                "link auth" => {
                    ad.link_auth = right.trim().to_string();
                }

                "BSSID" => {
                    ad.bssid = right.trim().to_string();
                }

                "SSID" => {
                    ad.ssid = right.trim().to_string();
                }

                "MCS" => {
                    ad.mcs = right.trim().parse::<i32>().unwrap();
                }

                "guardInterval" => {
                    ad.guard_interval = right.trim().parse::<i32>().unwrap();
                }

                "NSS" => {
                    ad.nss = right.trim().parse::<i32>().unwrap();
                }

                "channel" => {
                    ad.channel = right.trim().to_string();
                }

                _ => {}
            }
        }
    }

    r_io_utils::print_output::<AirportData>(&ad, output_type);
}
