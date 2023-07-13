use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WData {
    // meta: Meta,
    resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resources {
    user: String,
    tty: String,
    from: String,
    login_at: String,
    idle: String,
    jcpu: String,
    pcpu: String,
    what: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    // let mut meta = Meta {};
    let mut resources = vec![];

    let mut line_num = 1;

    for line in handle.lines() {
        let sl = line.unwrap();

        if line_num < 3 {
            // TODO think about meta struct
            line_num += 1;
            continue;
        }

        let mut line_parts = sl.split_whitespace();

        let mut r = Resources {
            user: String::new(),
            tty: String::new(),
            from: String::new(),
            login_at: String::new(),
            idle: String::new(),
            jcpu: String::new(),
            pcpu: String::new(),
            what: String::new(),
        };

        r.user = String::from(line_parts.next().unwrap());

        r.tty = String::from(line_parts.next().unwrap());
        r.from = String::from(line_parts.next().unwrap());
        r.login_at = String::from(line_parts.next().unwrap());
        r.idle = String::from(line_parts.next().unwrap());
        r.jcpu = String::from(line_parts.next().unwrap());
        r.pcpu = String::from(line_parts.next().unwrap());
        r.what = String::from(line_parts.next().unwrap());

        resources.push(r);

        // println!("{}", sl);
    }

    r_io_utils::print_output::<WData>(
        &WData {
            // meta: meta,
            resources: resources,
        },
        output_type,
    );
}
