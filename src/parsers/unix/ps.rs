use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PsData {
    //meta: Meta,
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
    pub pid: i32,
    pub tty: String,
    pub stat: String,
    pub time: String,
    pub command: String,
    //optional columns
    pub ppid: Option<i32>,
    pub pgid: Option<i32>,
    pub winpid: Option<i32>,
    pub uid: Option<i32>,
    pub user: Option<String>,
    pub cpu: Option<f32>,
    pub mem: Option<f32>,
    pub vsz: Option<i32>,
    pub rss: Option<i32>,
    pub start: Option<String>,
}

pub fn parse(data: Option<String>) -> PsData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    //let mut meta = Meta {
    //};
    let mut resources = vec![];
    let mut cols: Vec<String> = vec![];

    let mut line_one = true;

    for sl in buffer.lines() {
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
                _ => {}
            }
        }

        resources.push(r);
    }

    PsData {
        resources: resources,
    }
}
