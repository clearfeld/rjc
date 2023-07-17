use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerData {
    // meta: Meta,
    pub resources: Resources,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub prerelease: String,
    pub strict: bool,
    pub components: Vec<String>
}

fn split_minor_str(str: String) -> (String, String) {
    let mut string_split = (String::new(), String::new());
    let mut chars = str.chars();
    let starting_char = chars.nth(0).unwrap();

    let mut index = 1;
    for char in chars {
        if (char.is_alphabetic() && starting_char.is_numeric()) || (char.is_numeric() && starting_char.is_alphabetic()) {
            // minor string starts with a number.
            break;
        }
        index = index + 1;
    }
    let minor_split = str.split_at(index);
    if starting_char.is_numeric() {
        string_split.0 = String::from(minor_split.0);
        string_split.1 = String::from(minor_split.1);
    } 
    else {
        string_split.0 = String::from(minor_split.1);
        string_split.1 = String::from(minor_split.0);
    }

    string_split
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

pub fn parse(data: Option<String>) -> VerData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    //let mut resources = vec![];
    let mut resources = Resources {
        major: 0,
        minor: 0,
        patch: 0,
        prerelease: String::new(),
        strict: true,
        components: vec![]
    };

    for mut sl in buffer.lines() {
        if sl.starts_with('v') {
            sl = sl.trim_start_matches("v");
        }
        let mut line_parts = sl.trim().split(".");
        
        let component = line_parts.next().unwrap();
        resources.components.push(String::from(component));
        let major_str = component.parse::<i32>();
        if major_str.is_ok() {
            resources.major = major_str.unwrap();
        }
        else {
            resources.strict = false;
        }
        let next = line_parts.next();
        if next.is_some() {
            let minor_str = next.unwrap();
            resources.components.push(String::from(minor_str));
            if minor_str.parse::<i32>().is_ok() {
                resources.minor = minor_str.parse::<i32>().unwrap();
            }
            else {
                // if the minor str also has the prerelease info
                let minor_split = split_minor_str(String::from(minor_str));
                if minor_split.0.parse::<i32>().is_ok() {
                    resources.minor = minor_split.0.parse::<i32>().unwrap();
                } 
                else {
                    resources.strict = false;
                }
                resources.prerelease = minor_split.1;
            }
            // next iter might be the prerelease as well
            let mut patch = line_parts.next();
            if patch.is_some() {
                let patch_str = patch.unwrap();
                resources.components.push(String::from(patch_str));
                if patch_str.parse::<i32>().is_ok() {
                    resources.patch = patch_str.parse::<i32>().unwrap();
                } 
                else {
                    resources.prerelease = String::from(patch_str);
                    patch = line_parts.next();
                    if patch.is_some() {
                        resources.components.push(String::from(patch.unwrap()));
                        resources.patch = patch.unwrap().parse::<i32>().unwrap();

                    }
                }
            }
        }
        else {
            resources.strict = false;
        }
    }

    VerData {
        // meta: meta,
        resources: resources
    }
}