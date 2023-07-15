use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LsdData {
    // meta: Meta,
    pub resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub name: String,
    pub permissions: String,
    // links: i32,
    pub owner: String,
    pub group: String,
    pub date: String,

    pub size: f32,
    pub unit: String,
}

pub fn parse(data: Option<String>) -> LsdData {
    // let handle = io::stdin().lock();
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    let mut resources = vec![];

    for sl in buffer.lines() {
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

    LsdData {
        // meta: meta,
        resources: resources,
    }
}
