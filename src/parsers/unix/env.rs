use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvData {
    pub name: String,
    pub value: String,
}

pub fn parse(data: Option<String>) -> Vec<EnvData> {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    let mut envs = vec![];

    for sl in buffer.lines() {
        let equal_idx = sl.find("=").unwrap();

        envs.push(EnvData {
            name: String::from(&sl[..equal_idx]),
            value: String::from(&sl[equal_idx + 1..]),
        });
    }

    envs
}
