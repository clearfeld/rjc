use core::panic;

use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct DuData {
    pub size: i32,
    pub name: String,
}

pub fn parse(data: Option<String>) -> Vec<DuData> {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    let mut du = vec![];

    for sl in buffer.lines() {
        // println!("{}", sl);

        let space_idx = match sl.find("\t") {
            Some(v) => v,
            None => {
                panic!("Tab not found")
            }
        };

        du.push(DuData {
            size: sl[..space_idx].trim().parse::<i32>().unwrap(),
            name: String::from(sl[space_idx..].trim()),
        });
    }

    du
}
