use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailAddressData {
    pub username: String,
    pub domain: String,
    pub local: String,
    pub local_plus_suffix: Option<String>
}

pub fn parse(data: Option<String>) -> Vec<EmailAddressData> {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    let mut emails = vec![];

    for sl in buffer.lines() {
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

    emails
}
