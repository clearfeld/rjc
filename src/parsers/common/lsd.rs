use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LsdData {
    // meta: Meta,
    resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resources {
    name: String,
    permissions: String,
    // links: i32,
    owner: String,
    group: String,
    date: String,

    size: f32,
    unit: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    // let mut meta = Meta {};
    let mut resources = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();

        let mut line_parts = sl.split_whitespace();

        let mut r = Resources {
            name: String::new(),
            permissions: String::new(),
            // links: 0,
            owner: String::new(),
            group: String::new(),
            date: String::new(),

            size: 0.0,
            unit: String::new(),
        };

        r.permissions = String::from(line_parts.next().unwrap());
        // TODO: for certain permissions like l--------
        // should record where it points to

        // r.links = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.owner = String::from(line_parts.next().unwrap());
        r.group = String::from(line_parts.next().unwrap());

        r.size = line_parts.next().unwrap().parse::<f32>().unwrap();
        r.unit = String::from(line_parts.next().unwrap());

        r.date = String::from(line_parts.next().unwrap());
        r.date.push_str(" ");
        r.date.push_str(line_parts.next().unwrap());
        r.date.push_str(" ");
        r.date.push_str(line_parts.next().unwrap());
        r.date.push_str(" ");
        r.date.push_str(line_parts.next().unwrap());
        r.date.push_str(" ");
        r.date.push_str(line_parts.next().unwrap());

        r.name = String::from(line_parts.next().unwrap());

        resources.push(r);

        // println!("{}", sl);
    }

    r_io_utils::print_output::<LsdData>(
        &LsdData {
            // meta: meta,
            resources: resources,
        },
        output_type,
    );
}
