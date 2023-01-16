use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::args;

#[derive(Debug, Serialize, Deserialize)]
struct DirData {
    meta: Meta,
    resources: Vec<Resources>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    drive: String,
    serial: String,
    directory: String,
    files: i32,
    directories: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resources {
    date: String,
    time: String,
    is_dir: bool,
    size: Option<i32>,
    name: String,
    // TODO: maybe add this for jc compatibility
    // epoch: i32,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut meta = Meta {
        drive: String::new(),
        serial: String::new(),
        directory: String::new(),
        files: -1,
        directories: -1,
    };
    let mut resources = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();

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

    match output_type {
        args::OutputTypes::Yaml => {
            let output = serde_yaml::to_string(&&DirData {
                meta: meta,
                resources: resources,
            });

            match output {
                Ok(o) => {
                    println!("\n{}", o);
                }
                Err(e) => {
                    println!("Err - {:?}", e);
                }
            }
        }

        args::OutputTypes::Json => {
            let output = serde_json::to_string(&&DirData {
                meta: meta,
                resources: resources,
            });

            match output {
                Ok(o) => {
                    println!("\n{}", o);
                }
                Err(e) => {
                    println!("Err - {:?}", e);
                }
            }
        }
    }
}
