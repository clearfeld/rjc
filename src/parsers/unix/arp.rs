use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArpData {
    // meta: Meta,
    pub resources: Vec<Resources>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Meta {
// TODO:
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub name: Option<String>,
    pub address: String,
    pub hwtype: String,
    pub hwaddress: String,
    pub flags_mask: Option<String>,
    pub iface: String,
}

pub fn parse(data: Option<String>) -> ArpData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    // let mut meta = Meta {};
    let mut resources = vec![];

    let mut line_one = true;
    let mut bsd_style = true;

    for sl in buffer.lines() {
        if line_one {
            line_one = false;
            if sl.replace(" ", "") == "AddressHWtypeHWaddressFlagsMaskIface" {
                bsd_style = false;
                continue;
            }
        }

        let mut line_parts = sl.split_whitespace();

        let mut r = Resources {
            name: None,
            address: String::new(),
            hwtype: String::new(),
            hwaddress: String::new(),
            flags_mask: None,
            iface: String::new(),
        };
        if bsd_style {
            let name = String::from(line_parts.next().unwrap());
            if name != "?" {
                r.name = Some(name);
            }
            let mut address = String::from(line_parts.next().unwrap());
            address = String::from(address.trim_start_matches("("));
            address = String::from(address.trim_end_matches(")"));
            r.address = address;
            line_parts.next(); // `at`
            r.hwaddress = String::from(line_parts.next().unwrap());
            let mut hwtype = String::from(line_parts.next().unwrap());
            hwtype = String::from(hwtype.trim_start_matches("["));
            hwtype = String::from(hwtype.trim_end_matches("]"));
            r.hwtype = hwtype;
            line_parts.next(); // `on`
            r.iface = String::from(line_parts.next().unwrap());
        }
        else {
            r.address = String::from(line_parts.next().unwrap());
            r.hwtype = String::from(line_parts.next().unwrap());
            r.hwaddress = String::from(line_parts.next().unwrap());
            r.flags_mask = Some(String::from(line_parts.next().unwrap()));
            r.iface = String::from(line_parts.next().unwrap());
        }

        resources.push(r);
    }

    ArpData {
        // meta: meta,
        resources: resources,
    }
}
