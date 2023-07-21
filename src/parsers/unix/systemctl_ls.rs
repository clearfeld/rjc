use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemctlLSData {
    // meta: Meta,
    pub resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub listen: String,
    pub unit: String,
    pub activates: String
}

pub fn parse(data: Option<String>) -> SystemctlLSData {
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
            listen: String::new(),
            unit: String::new(),
            activates: String::new(),
        };
        let listen = line_parts.next();
        if listen.is_none() {
            break;
        }

        r.listen = String::from(listen.unwrap());
        r.unit = String::from(line_parts.next().unwrap());
        r.activates = String::from(line_parts.next().unwrap());

        resources.push(r);
    }

    SystemctlLSData {
        // meta: meta,
        resources: resources,
    }
}
