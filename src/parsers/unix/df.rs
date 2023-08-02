use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DfData {
    filesystem: String,
    one_k_blocks: i64,
    used: i64,
    available: i64,
    use_percent: i16,
    mounted_on: String
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut drives = vec![];
    let mut lines = handle.lines();

    let separator_line = lines.next().unwrap().unwrap();
    let mut separators = separator_line.split_whitespace();

    let filesystem_space = separators.next().unwrap().len();
    let mut one_k_blocks_space = separators.next().unwrap().len();
    let mut used_space = separators.next().unwrap().len();
    let mut available_space = separators.next().unwrap().len();
    let mut use_space = separators.next().unwrap().len();
    // let mut use_space = separators.next().unwrap().len();

    one_k_blocks_space = filesystem_space + 1 + one_k_blocks_space;
    used_space = one_k_blocks_space + 1 + used_space;
    available_space = used_space + 1 + available_space;
    use_space = available_space + 1 + use_space;

    for line in lines {
        let sl = line.unwrap();

        let mut line_parts = sl.split("\t");
        println!("{:?}", line_parts.next());

    //     println!("{}, {}, {}, {}, {}, {}", sl[..filesystem_space].trim(),
    //         sl[filesystem_space..one_k_blocks_space].trim(),
    //         sl[one_k_blocks_space..used_space].trim(),
    //         sl[used_space..available_space].trim(),
    //         sl[available_space..use_space-1].trim(),
    //         sl[use_space..].trim()
    // );

    //     drives.push(DfData {
    //         filesystem: String::from(sl[..filesystem_space].trim()),
    //         one_k_blocks: sl[filesystem_space..one_k_blocks_space].trim().parse::<i64>().unwrap(),
    //         used: sl[one_k_blocks_space..used_space].trim().parse::<i64>().unwrap(),
    //         available: sl[used_space..available_space].trim().parse::<i64>().unwrap(),
    //         use_percent: sl[available_space..use_space-1].trim().parse::<i16>().unwrap(),
    //         mounted_on: String::from(sl[use_space..].trim()),
    //     });

        // println!("{}", sl);
    }

    r_io_utils::print_output::<Vec<DfData>>(
        &drives,
        output_type,
    );
}
