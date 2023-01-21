use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize)]
struct TasklistData {
    image_name: String,
    pid: i32,
    session_name: String,
    session: i32,
    memory_usage: i32,

    // is this needed at all?
    // memory_unit: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut tasks = vec![];

    let mut lines = handle.lines();

    // Skip empty line and coloumn titles
    lines.next();
    lines.next();

    // Get separators line
    let separator_line = lines.next().unwrap().unwrap();
    let mut separators = separator_line.split_whitespace();

    let image_name_space = separators.next().unwrap().len();
    let mut pid_space = separators.next().unwrap().len();
    let mut session_space = separators.next().unwrap().len();
    let mut session_number_space = separators.next().unwrap().len();

    pid_space = image_name_space + 1 + pid_space;
    session_space = pid_space + 1 + session_space;
    session_number_space = session_space + 1 + session_number_space;

    for line in lines {
        let sl = line.unwrap();

        tasks.push(TasklistData{
            image_name: String::from(sl[..image_name_space].trim()),
            pid: sl[image_name_space..pid_space].trim().parse::<i32>().unwrap(),
            session_name: String::from(sl[pid_space..session_space].trim()),
            session: sl[session_space..session_number_space].trim().parse::<i32>().unwrap(),
            memory_usage: sl[session_number_space..sl.len()-1].trim().replace(",", "").parse::<i32>().unwrap(),
            // memory_unit: String::from(sl[sl.len()-2..sl.len()].trim()),
        });
    }

    r_io_utils::print_output::<Vec<TasklistData>>(
        &tasks,
        output_type,
    );
}
