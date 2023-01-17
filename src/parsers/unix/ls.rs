use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::args;

#[derive(Debug, Serialize, Deserialize)]
struct LsData {
    // meta: Meta,
    resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize)]
struct Resources {
    name: String,
    permissions: String,
    links: i32,
    owner: String,
    group: String,
    size: i32,
    date: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    // let mut meta = Meta {};
    let mut resources = vec![];

    let mut line_one = true;

    for line in handle.lines() {
        let sl = line.unwrap();

        if line_one {
            // TODO think about meta struct
            line_one = false;
            continue;
        }

        let mut line_parts = sl.split_whitespace();

        let mut r = Resources {
            name: String::new(),
            permissions: String::new(),
            links: 0,
            owner: String::new(),
            group: String::new(),
            size: 0,
            date: String::new(),
        };

        r.permissions = String::from(line_parts.next().unwrap());
        // TODO: for certain permissions like l--------
        // should record where it points to

        r.links = line_parts.next().unwrap().parse::<i32>().unwrap();
        r.owner = String::from(line_parts.next().unwrap());
        r.group = String::from(line_parts.next().unwrap());
        r.size = line_parts.next().unwrap().parse::<i32>().unwrap();

        r.date = String::from(line_parts.next().unwrap());
        r.date.push_str(" ");
        r.date.push_str(line_parts.next().unwrap());
        r.date.push_str(" ");
        r.date.push_str(line_parts.next().unwrap());

        r.name = String::from(line_parts.next().unwrap());

        resources.push(r);

        // println!("{}", sl);
    }

    match output_type {
        args::OutputTypes::Yaml => {
            match serde_yaml::to_string(&&LsData {
                // meta: meta,
                resources: resources,
            }) {
                Ok(o) => {
                    println!("\n{}", o);
                }
                Err(e) => {
                    println!("Err - {:?}", e);
                }
            }
        }

        args::OutputTypes::Json => {
            match serde_json::to_string(&&LsData {
                // meta: meta,
                resources: resources,
            }) {
                Ok(o) => {
                    println!("\n{}", o);
                }
                Err(e) => {
                    println!("Err - {:?}", e);
                }
            }
        }

        args::OutputTypes::Toml => {
            match toml::to_string(&&LsData {
                // meta: meta,
                resources: resources,
            }) {
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
