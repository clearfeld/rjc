use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ShadowData {
    // meta: Meta,
    resources: Vec<Resources>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resources {
    username: String,
    password: String,
    last_changed: i32,
    minimum: i32,
    maximum: i32,
    warn: i32,
    inactive: Option<i32>,
    expire: Option<i32>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    // let mut meta = Meta {};
    let mut resources = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();
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

    r_io_utils::print_output::<ShadowData>(
        &ShadowData {
            // meta: meta,
            resources: resources
        },
        output_type,
    );
}