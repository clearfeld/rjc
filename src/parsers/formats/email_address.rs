use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EmailAddressData {
    username: String,
    domain: String,
    local: String,
    local_plus_suffix: Option<String>
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut emails = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();

        let at_idx = sl.find("@").unwrap();

        match sl.find("+") {
            Some(plus_idx) => {
                emails.push(EmailAddressData{
                    username: String::from(&sl[..plus_idx]),
                    domain: String::from(&sl[at_idx+1..]),
                    local: String::from(&sl[..at_idx]),
                    local_plus_suffix: Some(String::from(&sl[plus_idx+1..at_idx]))
                });
            }

            None => {
                emails.push(EmailAddressData{
                    username: String::from(&sl[..at_idx]),
                    domain: String::from(&sl[at_idx+1..]),
                    local: String::from(&sl[..at_idx]),
                    local_plus_suffix: None
                });
            }
        }
    }

    r_io_utils::print_output::<Vec<EmailAddressData>>(
        &emails,
        output_type,
    );
}
