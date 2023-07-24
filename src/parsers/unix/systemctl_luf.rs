use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemctlLUFData {
    // meta: Meta,
    pub resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub unit_file: String,
    pub state: String,
    pub vendor_present: Option<String>
}

pub fn parse(data: Option<String>) -> SystemctlLUFData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    let mut resources = vec![];

    let mut line_one = true;

    for sl in buffer.lines() {
        if line_one {
            // TODO think about meta struct
            line_one = false;
            continue;
        }

        let mut line_parts = sl.split_whitespace();

        let mut r = Resources {
            unit_file: String::new(),
            state: String::new(),
            vendor_present: None,
        };
        let listen = line_parts.next();
        if listen.is_none() {
            break;
        }

        r.unit_file = String::from(listen.unwrap());
        r.state = String::from(line_parts.next().unwrap());
        let vendor_present = line_parts.next();

        if vendor_present.is_some() {
            if vendor_present.unwrap().trim() != "-" {
                r.vendor_present = Some(String::from(vendor_present.unwrap()));
            }
        }

        resources.push(r);
    }

    SystemctlLUFData {
        // meta: meta,
        resources: resources,
    }
}
