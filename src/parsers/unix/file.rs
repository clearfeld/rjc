use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileData {
    pub name: String,
    pub value: String,
}

pub fn parse(data: Option<String>) -> Vec<FileData> {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    let mut files = vec![];

    for sl in buffer.lines() {
        let colon_idx = sl.find(":").unwrap();

        files.push(FileData {
            name: String::from(&sl[..colon_idx]),
            value: String::from(sl[colon_idx + 1..].trim()),
        });
    }

    files
}
