use std::io::Read;
use std::io::{self};

use crate::args;

pub fn print_output<T: serde::Serialize>(
    command_data_struct: &T,
    output_type: args::OutputTypes
) -> () {
    match output_type {
        args::OutputTypes::Json => {
            match serde_json::to_string(command_data_struct) {
                Ok(o) => {
                    println!("\n{}", o);
                }
                Err(e) => {
                    println!("Err - {:?}", e);
                }
            }
        }

        args::OutputTypes::Pretty => {
            match serde_json::to_string_pretty(command_data_struct) {
                Ok(o) => {
                    println!("\n{}", o);
                }
                Err(e) => {
                    println!("Err - {:?}", e);
                }
            }
        }

        args::OutputTypes::Yaml => {
            match serde_yaml::to_string(command_data_struct) {
                Ok(o) => {
                    println!("\n{}", o);
                }
                Err(e) => {
                    println!("Err - {:?}", e);
                }
            }
        }

        args::OutputTypes::Toml => {
            match toml::to_string(command_data_struct) {
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

/// Determines whether a raw command data string was passed in (lib) or to use stdin lock to get the piped data (bin).
pub fn determine_data_source(data: Option<String>, buffer: &mut String) {
    match data {
        Some(val) => {
            *buffer = val;
        },

        None => {
            let mut h = io::stdin().lock();
            h.read_to_string(buffer).expect("Failed: To read string to buffer from stdin.");
        }
    }
}