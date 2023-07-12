use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PasswdData {
    // meta: Meta,
    resources: Vec<Resources>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resources {
    username: String,
    password: String,
    uid: i32,
    gid: i32,
    comment: String,
    home: String,
    shell: String,
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
            uid: 0,
            gid: 0,
            comment: String::new(),
            home: String::new(),
            shell: String::new(),
        };
        r.username = String::from(line_parts.next().unwrap());
        r.password = String::from(line_parts.next().unwrap());
        r.uid = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.gid = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.comment = String::from(line_parts.next().unwrap());
        r.home = String::from(line_parts.next().unwrap());
        r.shell = String::from(line_parts.next().unwrap());

        resources.push(r);
    }

    r_io_utils::print_output::<PasswdData>(
        &PasswdData {
            // meta: meta,
            resources: resources
        },
        output_type,
    );
}
