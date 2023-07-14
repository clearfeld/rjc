use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeAssociation {
    pub filetype: String,
    pub program: String,
}

pub fn parse(data: Option<String>) -> Vec<FileTypeAssociation> {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    let mut fta = vec![];

    for sl in buffer.lines() {
        let split_idx = sl.find("=").unwrap();

        fta.push(FileTypeAssociation{
            filetype: String::from(&sl[1..split_idx]),
            program: String::from(&sl[split_idx+1..])
        });

        // println!("{}", sl);
    }

    fta
}
