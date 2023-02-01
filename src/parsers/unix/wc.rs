use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WcData {
    meta: Meta,
    resources: Vec<Resources>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Meta {
    total_lines: i32,
    total_words: i32,
    total_characters: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resources {
    name: String,
    lines: i32,
    words: i32,
    characters: i32,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut meta = Meta {
        total_lines: 0,
        total_words: 0,
        total_characters: 0,
    };
    let mut resources = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();

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

    r_io_utils::print_output::<WcData>(
        &WcData {
            meta: meta,
            resources: resources,
        },
        output_type,
    );
}
