use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PsData {
    //meta: Meta,
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
    pid: i32,
    tty: String,
    stat: String,
    time: String,
    command: String,
    //optional columns
    ppid: Option<i32>,
    pgid: Option<i32>,
    winpid: Option<i32>,
    uid: Option<i32>,
    user: Option<String>,
    cpu: Option<f32>,
    mem: Option<f32>,
    vsz: Option<i32>,
    rss: Option<i32>,
    start: Option<String>,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    //let mut meta = Meta {
    //};
    let mut resources = vec![];
    let mut cols: Vec<String> = vec![];

    let mut line_one = true;

    for line in handle.lines() {
        let sl = line.unwrap();

        let line_parts = sl.split_whitespace();

        if line_one {
            // TODO think about meta struct
            cols = line_parts.map(str::to_string).collect();
            line_one = false;
            continue;
        }

        let mut r = Resources {
            pid: 0,
            tty: String::new(),
            stat: String::new(),
            time: String::new(),
            command: String::new(),
            ppid: None,
            pgid: None,
            winpid: None,
            uid: None,
            user: None,
            cpu: None,
            mem: None,
            vsz: None,
            rss: None,
            start: None,
        };

        for (index, part) in line_parts.enumerate() {
            match cols[index].as_str() {
                // columns that always show
                "PID" => {
                    r.pid = part.parse::<i32>().unwrap();
                }
                "TTY" => {
                    r.tty = part.to_string();
                }
                "STAT" => {
                    r.stat = part.to_string();
                }
                "TIME" => {
                    r.time = part.to_string();
                }
                "STIME" => {
                    r.time = part.to_string();
                }
                "COMMAND" => {
                    r.command = part.to_string();
                }
                // these don't always show
                "PPID" => {
                    r.ppid = Some(part.parse::<i32>().unwrap());
                }
                "PGID" => {
                    r.pgid = Some(part.parse::<i32>().unwrap());
                }
                "WINPID" => {
                    r.winpid = Some(part.parse::<i32>().unwrap());
                }
                "UID" => {
                    r.uid = Some(part.parse::<i32>().unwrap());
                }
                "USER" => {
                    r.user = Some(part.to_string());
                }
                "%CPU" => {
                    r.cpu = Some(part.parse::<f32>().unwrap());
                }
                "%MEM" => {
                    r.mem = Some(part.parse::<f32>().unwrap());
                }
                "VSZ" => {
                    r.vsz = Some(part.parse::<i32>().unwrap());
                }
                "RSS" => {
                    r.rss = Some(part.parse::<i32>().unwrap());
                }
                "STARTED" => {
                    r.start = Some(part.to_string());
                }
                "START" => {
                    r.start = Some(part.to_string());
                }
                _ => {
                }
            }
        }

        resources.push(r);
    }

    r_io_utils::print_output::<PsData>(
        &PsData {
            resources: resources,
        },
        output_type,
    );
}
