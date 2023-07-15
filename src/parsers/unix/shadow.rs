use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShadowData {
    // meta: Meta,
    pub resources: Vec<Resources>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub username: String,
    pub password: String,
    pub last_changed: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub warn: i32,
    pub inactive: Option<i32>,
    pub expire: Option<i32>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

pub fn parse(data: Option<String>) -> ShadowData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    let mut resources = vec![];

    for sl in buffer.lines() {
        let mut line_parts = sl.trim().split(":");

        let mut r = Resources {
            username: String::new(),
            password: String::new(),
            last_changed: 0,
            minimum: 0,
            maximum: 0,
            warn: 0,
            inactive: None,
            expire: None,
        };
        r.username = String::from(line_parts.next().unwrap());
        r.password = String::from(line_parts.next().unwrap());
        r.last_changed = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.minimum = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.maximum = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.warn = line_parts.next().unwrap().parse::<i32>().unwrap();
        let inactive = line_parts.next().unwrap();
        if !inactive.is_empty() {
            r.inactive = Some(inactive.parse::<i32>().unwrap());
        }
        let expire = line_parts.next().unwrap();
        if !expire.is_empty() {
            r.expire = Some(expire.parse::<i32>().unwrap());
        }

        resources.push(r);
    }

    ShadowData {
        // meta: meta,
        resources: resources
    }
}