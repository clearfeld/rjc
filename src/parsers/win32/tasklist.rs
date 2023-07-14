use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct TasklistData {
    pub image_name: String,
    pub pid: i32,
    pub session_name: String,
    pub session: i32,
    pub memory_usage: i32,

    // is this needed at all?
    // memory_unit: String,
}

pub fn parse(data: Option<String>) -> Vec<TasklistData> {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    let mut tasks = vec![];

    let mut lines = buffer.lines();

    // Skip empty line and coloumn titles
    lines.next();
    lines.next();

    // Get separators line
    let separator_line = lines.next().unwrap();
    let mut separators = separator_line.split_whitespace();

    let image_name_space = separators.next().unwrap().len();
    let mut pid_space = separators.next().unwrap().len();
    let mut session_space = separators.next().unwrap().len();
    let mut session_number_space = separators.next().unwrap().len();

    pid_space = image_name_space + 1 + pid_space;
    session_space = pid_space + 1 + session_space;
    session_number_space = session_space + 1 + session_number_space;

    for sl in lines {
        tasks.push(TasklistData{
            image_name: String::from(sl[..image_name_space].trim()),
            pid: sl[image_name_space..pid_space].trim().parse::<i32>().unwrap(),
            session_name: String::from(sl[pid_space..session_space].trim()),
            session: sl[session_space..session_number_space].trim().parse::<i32>().unwrap(),
            memory_usage: sl[session_number_space..sl.len()-1].trim().replace(",", "").parse::<i32>().unwrap(),
            // memory_unit: String::from(sl[sl.len()-2..sl.len()].trim()),
        });
    }

    tasks
}
