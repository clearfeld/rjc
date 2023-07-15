use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChageData {
    pub name: String,
    pub value: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

pub fn parse(data: Option<String>) -> Vec<ChageData> {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    let mut resources = vec![];

    for sl in buffer.lines() {
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

    resources
}
