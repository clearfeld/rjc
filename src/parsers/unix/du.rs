use core::panic;
use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize)]
struct DuData {
    size: i32,
    name: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut du = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();
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

    r_io_utils::print_output::<Vec<DuData>>(&du, output_type);
}
