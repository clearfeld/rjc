use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeData {
    // pub meta: Meta,
    pub resources: Resources,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub real_time: f32,
    pub user_time: f32,
    pub system_time: f32,
    pub elapsed_time: String,
    pub elapsed_time_hours: i32,
    pub elapsed_time_minutes: i32,
    pub elapsed_time_seconds: i32,
    pub elapsed_time_centiseconds: i32,
    pub elapsed_time_total_seconds: f32,
    pub cpu_percent: i32,
    pub average_shared_text_size: i32,
    pub average_unshared_data_size: i32,
    pub average_unshared_stack_size: i32,
    pub average_shared_memory_size: i32,
    pub maximum_resident_set_size: i32,
    pub block_input_operations: i32,
    pub block_output_operations: i32,
    pub major_pagefaults: i32,
    pub minor_pagefaults: i32,
    pub swaps: i32,
    pub page_reclaims: i32,
    pub page_faults: i32,
    pub messages_sent: i32,
    pub message_received: i32,
    pub signals_received: i32,
    pub voluntary_context_switches: i32,
    pub involuntary_context_switches: i32,
    pub command_being_timed: String,
    pub average_stack_size: i32,
    pub average_total_size: i32,
    pub average_resident_set_size: i32,
    pub signals_delivered: i32,
    pub page_size: i32,
    pub exit_status: i32,
}

pub fn parse(data: Option<String>) -> TimeData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};

    let mut r = Resources {
        real_time: 0.0,
        user_time: 0.0,
        system_time: 0.0,
        elapsed_time: String::new(),
        elapsed_time_hours: 0,
        elapsed_time_minutes: 0,
        elapsed_time_seconds: 0,
        elapsed_time_centiseconds: 0,
        elapsed_time_total_seconds: 0.0,
        cpu_percent: 0,
        average_shared_text_size: 0,
        average_unshared_data_size: 0,
        average_unshared_stack_size: 0,
        average_shared_memory_size: 0,
        maximum_resident_set_size: 0,
        block_input_operations: 0,
        block_output_operations: 0,
        major_pagefaults: 0,
        minor_pagefaults: 0,
        swaps: 0,
        page_reclaims: 0,
        page_faults: 0,
        messages_sent: 0,
        message_received: 0,
        signals_received: 0,
        voluntary_context_switches: 0,
        involuntary_context_switches: 0,
        command_being_timed: String::new(),
        average_stack_size: 0,
        average_total_size: 0,
        average_resident_set_size: 0,
        signals_delivered: 0,
        page_size: 0,
        exit_status: 0,
    };

    for line in buffer.lines() {
        let mut sl = String::from(line.trim());
        let mut field= String::from(sl.split_once(":").unwrap().0);
        field.push_str(": ");
        sl = String::from(sl.trim_start_matches(&field));

        match field.as_str() {
            "Command being timed: " => {r.command_being_timed = sl}
            "User time (seconds): " => {r.user_time = sl.parse::<f32>().unwrap()}
            "System time (seconds): " => {r.system_time = sl.parse::<f32>().unwrap()}
            "Percent of CPU this job got: " => {
                sl.pop();
                r.cpu_percent = sl.parse::<i32>().unwrap()
            }
            "Elapsed (wall clock) time (h: " => {
                sl = String::from(sl.trim_start_matches("Elapsed (wall clock) time (h:mm:ss or m:ss): "));
                let mut time_split = sl.split(":");
                let format_size = time_split.clone().count();
                if format_size == 3 {
                    let hours = time_split.next().unwrap();
                    r.elapsed_time_hours = hours.parse::<i32>().unwrap();
                }
                let minutes = time_split.next().unwrap();
                let seconds_and_centiseconds = time_split.next().unwrap();
                let seconds_split = seconds_and_centiseconds.split_once(".").unwrap();

                r.elapsed_time_minutes = minutes.parse::<i32>().unwrap();
                r.elapsed_time_seconds = seconds_split.0.parse::<i32>().unwrap();
                r.elapsed_time_centiseconds = seconds_split.1.parse::<i32>().unwrap();
                r.elapsed_time_total_seconds = seconds_and_centiseconds.parse::<f32>().unwrap() + (r.elapsed_time_minutes * 60) as f32 + (r.elapsed_time_hours * 3600) as f32;
            
                r.elapsed_time = sl;
            }
            "Average shared text size (kbytes): " => {r.average_shared_text_size = sl.parse::<i32>().unwrap()}
            "Average unshared data size (kbytes): " => {r.average_unshared_data_size = sl.parse::<i32>().unwrap()}
            "Average stack size (kbytes): " => {r.average_stack_size = sl.parse::<i32>().unwrap()}
            "Average total size (kbytes): " => {r.average_total_size = sl.parse::<i32>().unwrap()}
            "Maximum resident set size (kbytes): " => {r.maximum_resident_set_size = sl.parse::<i32>().unwrap()}
            "Average resident set size (kbytes): " => {r.average_resident_set_size = sl.parse::<i32>().unwrap()}
            "Major (requiring I/O) page faults: " => {r.major_pagefaults = sl.parse::<i32>().unwrap()}
            "Minor (reclaiming a frame) page faults: " => {r.minor_pagefaults = sl.parse::<i32>().unwrap()}
            "Voluntary context switches: " => {r.voluntary_context_switches = sl.parse::<i32>().unwrap()}
            "Involuntary context switches: " => {r.involuntary_context_switches = sl.parse::<i32>().unwrap()}
            "Swaps: " => {r.swaps = sl.parse::<i32>().unwrap()}
            "File System inputs: " => {r.block_input_operations = sl.parse::<i32>().unwrap()}
            "File System outputs: " => {r.block_output_operations = sl.parse::<i32>().unwrap()}
            "Socket messages sent: : " => {r.messages_sent = sl.parse::<i32>().unwrap()}
            "Socket messages received: " => {r.message_received = sl.parse::<i32>().unwrap()}
            "Signals delivered: " => {r.signals_delivered = sl.parse::<i32>().unwrap()}
            "Page size (bytes): " => {r.page_size = sl.parse::<i32>().unwrap()}
            "Exit status: " => {r.exit_status = sl.parse::<i32>().unwrap()}
            _ => {}
        }
    }

    TimeData {
        // meta: meta,
        resources: r,
    }
}
