use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AirportData {
    #[serde(rename = "agrCtlRSSI")]
    pub agr_ctl_rssi: i32,
    #[serde(rename = "agrExtRSSI")]
    pub agr_ext_rssi: i32,
    #[serde(rename = "agrCtlNoise")]
    pub agr_ctl_noise: i32,
    #[serde(rename = "agrExtNoise")]
    pub agr_ext_noise: i32,
    pub state: String,
    #[serde(rename = "op mode")]
    pub op_mode: String,
    #[serde(rename = "lastTxRate")]
    pub last_tx_rate: i32,
    #[serde(rename = "maxRate")]
    pub max_rate: i32,
    #[serde(rename = "lastAssocStatus")]
    pub last_assoc_status: i32,
    #[serde(rename = "802.11 auth")]
    pub e_auth: String,
    #[serde(rename = "link auth")]
    pub link_auth: String,
    #[serde(rename = "BSSID")]
    pub bssid: String,
    #[serde(rename = "SSID")]
    pub ssid: String,
    #[serde(rename = "MCS")]
    pub mcs: i32,
    #[serde(rename = "guardInterval")]
    pub guard_interval: i32,
    #[serde(rename = "NSS")]
    pub nss: i32,
    pub channel: String,
}

pub fn parse(data: Option<String>) -> AirportData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

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

    for sl in buffer.lines() {

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

    ad
}
