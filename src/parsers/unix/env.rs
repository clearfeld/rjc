use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EnvData {
    name: String,
    value: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut envs = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();

        let equal_idx = sl.find("=").unwrap();

        envs.push(EnvData {
            name: String::from(&sl[..equal_idx]),
            value: String::from(&sl[equal_idx + 1..]),
        });
    }

    r_io_utils::print_output::<Vec<EnvData>>(
        &envs,
        output_type
    );
}
