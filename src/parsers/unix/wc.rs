use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WcData {
    pub meta: Meta,
    pub resources: Vec<Resources>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub total_lines: i32,
    pub total_words: i32,
    pub total_characters: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub name: String,
    pub lines: i32,
    pub words: i32,
    pub characters: i32,
}

pub fn parse(data: Option<String>) -> WcData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    let mut meta = Meta {
        total_lines: 0,
        total_words: 0,
        total_characters: 0,
    };
    let mut resources = vec![];

    for sl in buffer.lines() {
        let mut line_parts = sl.split_whitespace();

        let mut r = Resources {
            name: String::new(),
            lines: 0,
            words: 0,
            characters: 0,
        };

        r.lines = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.words = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.characters = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.name = String::from(line_parts.next().unwrap());

        resources.push(r);

        // println!("{}", sl);
    }
    let met = resources.pop().unwrap();
    meta.total_lines = met.lines;
    meta.total_words = met.words;
    meta.total_characters = met.characters;

    WcData {
        meta: meta,
        resources: resources,
    }
}
