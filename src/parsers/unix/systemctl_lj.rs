use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemctlLJData {
    // meta: Meta,
    pub resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub job: i32,
    pub unit: String,
    pub job_type: String,
    pub state: String
}

pub fn parse(data: Option<String>) -> SystemctlLJData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    let mut resources = vec![];

    let mut line_one = true;

    for sl in buffer.lines() {
        if line_one {
            // TODO think about meta struct
            line_one = false;
            continue;
        }

        let mut line_parts = sl.split_whitespace();

        let mut r = Resources {
            job: 0,
            unit: String::new(),
            job_type: String::new(),
            state: String::new(),
        };
        let job = line_parts.next();
        if job.is_none() {
            break;
        }

        r.job = job.unwrap().parse::<i32>().unwrap();
        r.unit = String::from(line_parts.next().unwrap());
        r.job_type = String::from(line_parts.next().unwrap());
        r.state = String::from(line_parts.next().unwrap());

        resources.push(r);
    }

    SystemctlLJData {
        // meta: meta,
        resources: resources,
    }
}
