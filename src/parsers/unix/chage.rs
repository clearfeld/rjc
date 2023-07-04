use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChageData {
    name: String,
    value: String,
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
        let space_idx = match sl.find("\t") {
            Some(v) => v,
            None => {
                panic!("Tab not found")
            }
        };
        let key = sl[..space_idx].trim();
        let val = sl[space_idx..].trim().split(":").last().unwrap().trim();

        resources.push(ChageData {
            name: String::from(key),
            value: String::from(val),
        });
        continue;
    }

    r_io_utils::print_output::<Vec<ChageData>>(
        &resources,
        output_type,
    );
}
