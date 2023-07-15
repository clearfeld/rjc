use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswdData {
    // meta: Meta,
    pub resources: Vec<Resources>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub username: String,
    pub password: String,
    pub uid: i32,
    pub gid: i32,
    pub comment: String,
    pub home: String,
    pub shell: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

pub fn parse(data: Option<String>) -> PasswdData {
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

    PasswdData {
        // meta: meta,
        resources: resources
    }
}
