use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WData {
    // pub meta: Meta,
    pub resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub user: String,
    pub tty: String,
    pub from: String,
    pub login_at: String,
    pub idle: String,
    pub jcpu: String,
    pub pcpu: String,
    pub what: String,
}

pub fn parse(data: Option<String>) -> WData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    let mut resources = vec![];

    let mut line_num = 1;

    for sl in buffer.lines() {
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

    WData {
        // meta: meta,
        resources: resources,
    }
}
