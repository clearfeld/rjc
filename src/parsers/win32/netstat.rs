use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

use crate::args;
use crate::r_io_utils;

// netstat -an

#[derive(Debug, Serialize, Deserialize)]
struct ConnectionsData {
    tcp: Vec<TCPConnectionData>,
    udp: Vec<UDPConnectionData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TCPConnectionData {
    // protocol: String,
    local_address: String,
    foreign_address: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UDPConnectionData {
    // protocol: String,
    local_address: String,
    foreign_address: String,
}

pub fn parse(output_type: args::OutputTypes) {
    let handle = io::stdin().lock();

    let mut tcp_connections = vec![];
    let mut udp_connections = vec![];

    for line in handle.lines() {
        let sl = line.unwrap();
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

    r_io_utils::print_output::<ConnectionsData>(
        &ConnectionsData{
            tcp: tcp_connections,
            udp: udp_connections
        },
        output_type,
    );
}
