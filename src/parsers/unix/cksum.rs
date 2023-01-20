use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CksumData {
    filename: String,
    checksum: i64,
    blocks: i32,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    // TODO: add flag to to not print cksum: none stdio outputs

    let mut cksums = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();

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

    r_io_utils::print_output::<Vec<CksumData>>(
        &cksums,
        output_type,
    );
}
