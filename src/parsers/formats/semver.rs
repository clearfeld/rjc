use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemverData {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub prerelease: Option<String>,
    pub build: Option<String>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

pub fn parse(data: Option<String>) -> SemverData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    //let mut resources = vec![];
    let mut resources = SemverData {
        major: 0,
        minor: 0,
        patch: 0,
        prerelease: None,
        build: None,
    };

    for sl in buffer.lines() {
        let mut line_parts = sl.trim().split(".");

        let component = line_parts.next().unwrap();

        let major_str = component.parse::<i32>();
        if major_str.is_ok() {
            resources.major = major_str.unwrap();
        }

        let next = line_parts.next();
        if next.is_some() {
            let minor_str = next.unwrap();
            if minor_str.parse::<i32>().is_ok() {
                resources.minor = minor_str.parse::<i32>().unwrap();
            }

            let patch = line_parts.next();
            if patch.is_some() {
                let mut patch_str = String::from(patch.unwrap());
                let optional_str = line_parts.next();
                if optional_str.is_some() {
                    patch_str = patch_str + "." + optional_str.unwrap();
                }
                let patch_parts = patch_str.split_once("-");

                if patch_parts.is_some() {
                    resources.patch = patch_parts.unwrap().0.parse::<i32>().unwrap();
                    let optional_split = patch_parts.unwrap().1.split_once("+");
                    if optional_split.is_some() {
                        resources.prerelease = Some(String::from(optional_split.unwrap().0));
                        resources.build = Some(String::from(optional_split.unwrap().1));
                    }
                    else {
                        resources.prerelease = Some(String::from(patch_parts.unwrap().1));
                    }
                }
                else {
                    let patch_parse = patch_str.parse::<i32>();
                    if patch_parse.is_ok() {
                        resources.patch = patch_parse.unwrap();
                    }
                }
            }
        }
    }

    resources
}