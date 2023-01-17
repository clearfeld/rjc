use crate::args;

pub fn print_output<T: serde::Serialize>(
    command_data_struct: &T,
    output_type: args::OutputTypes
) -> () {
    match output_type {
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