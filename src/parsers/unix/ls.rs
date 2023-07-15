use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LsData {
    // meta: Meta,
    pub resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub name: String,
    pub permissions: String,
    pub links: i32,
    pub owner: String,
    pub group: String,
    pub size: i32,
    pub date: String,
}

pub fn parse(data: Option<String>) -> LsData {
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
            name: String::new(),
            permissions: String::new(),
            links: 0,
            owner: String::new(),
            group: String::new(),
            size: 0,
            date: String::new(),
        };

        r.permissions = String::from(line_parts.next().unwrap());
        // TODO: for certain permissions like l--------
        // should record where it points to

        r.links = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.owner = String::from(line_parts.next().unwrap());
        r.group = String::from(line_parts.next().unwrap());
        r.size = line_parts.next().unwrap().parse::<i32>().unwrap();

        r.date = String::from(line_parts.next().unwrap());
        r.date.push_str(" ");
        r.date.push_str(line_parts.next().unwrap());
        r.date.push_str(" ");
        r.date.push_str(line_parts.next().unwrap());

        r.name = String::from(line_parts.next().unwrap());

        resources.push(r);

        // println!("{}", sl);
    }

    LsData {
        // meta: meta,
        resources: resources,
    }
}
