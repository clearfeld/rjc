use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemctlData {
    // meta: Meta,
    pub resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub unit: String,
    pub load: String,
    pub active: String,
    pub sub: String,
    pub description: String
}

pub fn parse(data: Option<String>) -> SystemctlData {
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
            unit: String::new(),
            load: String::new(),
            active: String::new(),
            sub: String::new(),
            description: String::new(),
        };
        let unit = line_parts.next();
        if unit.is_none() {
            break;
        }

        r.unit = String::from(unit.unwrap());
        r.load = String::from(line_parts.next().unwrap());
        r.active = String::from(line_parts.next().unwrap());
        r.sub = String::from(line_parts.next().unwrap());
        r.description = String::from(line_parts.next().unwrap());

        resources.push(r);
    }

    SystemctlData {
        // meta: meta,
        resources: resources,
    }
}
