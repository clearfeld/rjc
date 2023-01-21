use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileData {
    name: String,
    value: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut files = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();

        let colon_idx = sl.find(":").unwrap();

        files.push(FileData {
            name: String::from(&sl[..colon_idx]),
            value: String::from(sl[colon_idx + 1..].trim()),
        });
    }

    r_io_utils::print_output::<Vec<FileData>>(
        &files,
        output_type
    );
}
