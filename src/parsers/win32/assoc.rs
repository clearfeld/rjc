use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize)]
struct FileTypeAssociation {
    filetype: String,
    program: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut fta = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();

        let split_idx = sl.find("=").unwrap();

        fta.push(FileTypeAssociation{
            filetype: String::from(&sl[1..split_idx]),
            program: String::from(&sl[split_idx+1..])
        });

        // println!("{}", sl);
    }


    r_io_utils::print_output::<Vec<FileTypeAssociation>>(
        &fta,
        output_type,
    );
}
