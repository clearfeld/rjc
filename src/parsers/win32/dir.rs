use std::io::Read;
use std::io::{self};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirData {
    pub meta: Meta,
    pub resources: Vec<Resources>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub drive: String,
    pub serial: String,
    pub directory: String,
    pub files: i32,
    pub directories: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    pub date: String,
    pub time: String,
    pub is_dir: bool,
    pub size: Option<i32>,
    pub name: String,
    // TODO: maybe add this for jc compatibility
    // epoch: i32,
}

pub fn parse(data: Option<String>) -> DirData {
    let mut handle = String::new();
    match data {
        Some(val) => {
            handle = val;
        },

        None => {
            let mut h = io::stdin().lock();
            h.read_to_string(&mut handle).expect("Failed to read stdin");
        }
    };

    let mut meta = Meta {
        drive: String::new(),
        serial: String::new(),
        directory: String::new(),
        files: -1,
        directories: -1,
    };
    let mut resources = vec![];

    for sl in handle.lines() {
        if sl.starts_with(" ") {
            if sl.starts_with(" Volume in drive") {
                meta.drive = String::from(&sl[17..18]);
            } else if sl.starts_with(" Volume Serial Number") {
                meta.serial = String::from(&sl[25..]);
            } else if sl.starts_with(" Directory of ") {
                meta.directory = String::from(&sl[14..]);
            } else if sl.contains("File(s)") {
                match sl.find("File(s)") {
                    Some(idx) => {
                        meta.files = sl[..idx].trim().parse::<i32>().unwrap();
                    }

                    None => {
                        // TODO: error logging
                        meta.files = -1;
                    }
                }
            } else if sl.contains("Dir(s)") {
                match sl.find("Dir(s)") {
                    Some(idx) => {
                        meta.directories = sl[..idx].trim().parse::<i32>().unwrap();
                    }

                    None => {
                        // TODO: error logging
                        meta.directories = -1;
                    }
                }
            }
        } else if !sl.is_empty() {
            let mut r = Resources {
                date: String::new(),
                time: String::new(),
                is_dir: false,
                size: None,
                name: String::new(),
            };

            r.date = String::from(&sl[..10]);
            r.time = String::from(&sl[12..20]);

            r.is_dir = match sl.find("<DIR>") {
                Some(_) => true,
                None => false,
            };

            let mut t = sl[28..].split_whitespace();
            match t.next().unwrap().replace(",", "").parse::<i32>() {
                Ok(i) => r.size = Some(i),
                Err(_) => {}
            }

            r.name = String::from(sl[28..].split_whitespace().last().unwrap());

            resources.push(r);
        }

        // println!("{}", sl);
    }

    DirData {
        meta: meta,
        resources: resources,
    }
}

pub fn print(dd: DirData, output_type: args::OutputTypes) {
    r_io_utils::print_output::<DirData>(
        &dd,
        output_type,
    );
}