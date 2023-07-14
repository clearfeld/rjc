use serde::{Deserialize, Serialize};

use crate::r_io_utils;

// netstat -an

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionsData {
    pub tcp: Vec<TCPConnectionData>,
    pub udp: Vec<UDPConnectionData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TCPConnectionData {
    // protocol: String,
    pub local_address: String,
    pub foreign_address: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UDPConnectionData {
    // protocol: String,
    pub local_address: String,
    pub foreign_address: String,
}

pub fn parse(data: Option<String>) -> ConnectionsData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    let mut tcp_connections = vec![];
    let mut udp_connections = vec![];

    for sl in buffer.lines() {
        let slt = sl.trim();

        if slt.starts_with("T") {
            let mut iter = slt.split_whitespace();
            iter.next(); // skip protocol
            tcp_connections.push(TCPConnectionData{
                local_address: String::from(iter.next().unwrap()),
                foreign_address: String::from(iter.next().unwrap()),
                state: String::from(iter.next().unwrap()),
            });
        } else if slt.starts_with("U") {
            let mut iter = slt.split_whitespace();
            iter.next(); // skip protocol
            udp_connections.push(UDPConnectionData{
                local_address: String::from(iter.next().unwrap()),
                foreign_address: String::from(iter.next().unwrap()),
            });
        }
    }

    ConnectionsData{
        tcp: tcp_connections,
        udp: udp_connections
    }
}
