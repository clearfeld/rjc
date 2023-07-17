use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SysctlData {
    // pub meta: Meta,
    pub resources: HashMap<String, String>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Meta {
// TODO:
// }

pub fn parse(data: Option<String>) -> SysctlData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    let mut resources = HashMap::new();

    for sl in buffer.lines() {
        let k_v = String::from(sl.trim());
        let k_v_option = k_v.split_once(" = ");
        if k_v_option.is_some() {
            let k_v_split = k_v_option.unwrap();
            resources.insert(String::from(k_v_split.0), String::from(k_v_split.1));
        }
    }

    SysctlData {
        // meta: meta,
        resources: resources,
    }
}
