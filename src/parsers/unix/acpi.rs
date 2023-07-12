use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AcpiData {
    //meta: Meta,
    resources: Vec<Resources>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TripPoint {
    id: i32,
    switches_to_mode: String,
    temperature: f32,
    temperature_unit: char,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resources {
    // all have these
    id: i32,
    hardware_type: String,
    // Battery fields
    state: Option<String>,
    charge_percent: Option<i32>,
    until_charged: Option<String>,
    charge_remaining: Option<String>,
    design_capacity: Option<i32>,
    last_full_capacity: Option<i32>,
    last_full_capacity_percent: Option<i32>,
    charge_msg: Option<String>,
    // Adapter
    online: Option<bool>,
    // Thermal
    mode: Option<String>,
    temperature: Option<f32>,
    temperature_unit: Option<char>,
    trip_points: Vec<TripPoint>,
    // cooling
    messages: Vec<String>,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut resources = vec![];

    let mut current_obj = Resources {
        id: 0,
        hardware_type: String::new(),
        state: None,
        charge_percent: None,
        until_charged: None,
        charge_remaining: None,
        design_capacity: None,
        last_full_capacity: None,
        last_full_capacity_percent: None,
        charge_msg: None,
        online: None,
        mode: None,
        temperature: None,
        temperature_unit: None,
        trip_points: vec![],
        messages: vec![],
    };


    let mut current_line = 0;

    for line in handle.lines() {
        let sl = line.unwrap();
        let line_parts_2 = sl.split_once(":").unwrap();
        let hardware_str = line_parts_2.0;

        let  mut hardware_parts = hardware_str.split_whitespace();
        let hardware_type = hardware_parts.next().unwrap();

        let id_str = hardware_parts.next().unwrap().to_string();
        let hardware_id = id_str.parse::<i32>().unwrap();

        if current_obj.hardware_type != hardware_type || current_obj.id != hardware_id || current_obj.hardware_type.is_empty(){
            if !current_obj.hardware_type.is_empty() {
                resources.push(current_obj.clone());
                current_obj = Resources {
                    id: 0,
                    hardware_type: String::new(),
                    state: None,
                    charge_percent: None,
                    until_charged: None,
                    charge_remaining: None,
                    design_capacity: None,
                    last_full_capacity: None,
                    last_full_capacity_percent: None,
                    charge_msg: None,
                    online: None,
                    mode: None,
                    temperature: None,
                    temperature_unit: None,
                    trip_points: vec![],
                    messages: vec![],
                };
            }
            //New object
            current_obj.id = hardware_id;
            current_obj.hardware_type = String::from(hardware_type);
            current_line = 0;
        }
        else {
            current_line += 1;
        }
        match current_obj.hardware_type.as_str() {
            "Battery" => {
                match current_line {
                    0 => {
                        let mut hardware_info = line_parts_2.1.split(",");
                        let state = hardware_info.next().unwrap().to_string();
                        let mut charge_percent = hardware_info.next().unwrap().trim().to_string();

                        charge_percent.pop();

                        if state != " Full" {
                            let battery_state = hardware_info.next().unwrap().trim().to_string();
                            if battery_state.ends_with("remaining") {
                                current_obj.charge_remaining = Some(String::from(battery_state.split_once(" ").unwrap().0));
                            }
                            else if battery_state.ends_with("until charged") {
                                current_obj.until_charged = Some(String::from(battery_state.split_once(" ").unwrap().0));
                            }
                        }

                        current_obj.state = Some(state);
                        current_obj.charge_percent = Some(charge_percent.parse::<i32>().unwrap());
                    }
                    1 => {
                        // for design capacity and last full capacity
                        let mut hardware_info = line_parts_2.1.split(",");
                        let design_capacity = hardware_info.next().unwrap().to_string();
                        current_obj.design_capacity = Some(design_capacity.strip_prefix(" design capacity ").unwrap().strip_suffix(" mAh").unwrap().parse::<i32>().unwrap());
                        let last_capacity = hardware_info.next().unwrap().to_string();
                        let last_capacity_split = last_capacity.split_once(" mAh = ").unwrap();
                        let last_capacity_mah = String::from(last_capacity_split.0);
                        current_obj.last_full_capacity = Some(last_capacity_mah.strip_prefix(" last full capacity ").unwrap().parse::<i32>().unwrap());
                        let mut capacity_percent = String::from(last_capacity_split.1);
                        capacity_percent.pop();
                        current_obj.last_full_capacity_percent = Some(capacity_percent.parse::<i32>().unwrap());

                    }
                    _ => {
                    }
                }
            }
            "Adapter" => {                
                match current_line {
                    0 => {
                        current_obj.online = Some(line_parts_2.1 == " onl-line");
                    }
                    _ => {
                    }
                }
            }
            "Thermal" => {
                match current_line {
                    0 => {
                        let temp_info = line_parts_2.1.split_once(", ").unwrap();
                        current_obj.mode = Some(String::from(temp_info.0.trim()));
                        let temp_parts = temp_info.1.split_once(" degrees ").unwrap();
                        current_obj.temperature = Some(temp_parts.0.parse::<f32>().unwrap());
                        current_obj.temperature_unit = Some(temp_parts.1.parse::<char>().unwrap());
                    }
                    _ => {
                        let trip_point_string = String::from(String::from(line_parts_2.1).strip_prefix(" trip point ").unwrap());
                        let mut trip_point_parts = trip_point_string.split_whitespace();
                        let tp_id = trip_point_parts.next().unwrap().parse::<i32>().unwrap();
                        let mut trip_point_parts = trip_point_parts.skip(3);
                        let switches_to_mode = String::from(trip_point_parts.next().unwrap());
                        let mut trip_point_parts = trip_point_parts.skip(2);
                        let temperature = trip_point_parts.next().unwrap().parse::<f32>().unwrap();
                        let mut trip_point_parts = trip_point_parts.skip(1);
                        let unit = trip_point_parts.next().unwrap().parse::<char>().unwrap();
                        current_obj.trip_points.push(TripPoint{id: tp_id, switches_to_mode: switches_to_mode, temperature: temperature, temperature_unit: unit});
                    }
                }
            }
            "Cooling" => {
                let cooling_info = line_parts_2.1;
                current_obj.messages.push(String::from(cooling_info));
            }
            _ => {
            }
        }
    }

    r_io_utils::print_output::<AcpiData>(
        &AcpiData {
            resources: resources,
        },
        output_type,
    );
}

