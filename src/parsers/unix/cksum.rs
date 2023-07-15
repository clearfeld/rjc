use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CksumData {
    pub filename: String,
    pub checksum: i64,
    pub blocks: i32,
}

pub fn parse(data: Option<String>) -> Vec<CksumData> {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // TODO: add flag to to not print cksum: none stdio outputs

    let mut cksums = vec![];

    for sl in buffer.lines() {
        if sl.starts_with("c") {
            // TODO: should this be recorded in a meta struct
            // println!("cksum: issue or directory");
            continue;
        }

        let mut line_parts = sl.split_whitespace();

        cksums.push(CksumData {
            checksum: line_parts.next().unwrap().parse::<i64>().unwrap(),
            blocks: line_parts.next().unwrap().parse::<i32>().unwrap(),
            filename: String::from(line_parts.next().unwrap()),
        });

        // println!("{}", sl);
    }

    cksums
}
