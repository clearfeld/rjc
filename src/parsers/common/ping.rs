use serde::{Deserialize, Serialize};

use crate::r_io_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct PingData {
    pub destination_ip: String,
    pub data_bytes: i32,
    pub pattern: Option<String>,
    pub destination: String,
    pub packets_transmitted: i32,
    pub packets_received: i32,
    pub packet_loss_percent: f64,
    pub duplicates: i32,
    pub round_trip_as_min: f64,
    pub round_trip_as_avg: f64,
    pub round_trip_as_max: f64,
    pub round_trip_as_stddev: f64,
    pub total_time_ms: i32,
    pub responses: Vec<Response>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Response {
    pub response_type: String,
    pub unparsed_line: String,
    pub timestamp: f64,
    pub bytes: i32,
    pub response_ip: String,
    pub icmp_seq: i32,
    pub ttl: i32,
    pub time_ms: f64,
    pub duplicate: bool,
}

pub fn parse(data: Option<String>) -> PingData {
    let mut buffer = String::new();
    // TODO(clearfeld): probably should add some stronger checks when determining data source
    r_io_utils::determine_data_source(data, &mut buffer);

    let mut data = PingData {
        destination_ip: String::new(),
        data_bytes: 0,
        pattern: None,
        destination: String::new(),
        packets_transmitted: 0,
        packets_received: 0,
        packet_loss_percent: 0.0,
        duplicates: 0,
        round_trip_as_min: 0.0,
        round_trip_as_avg: 0.0,
        round_trip_as_max: 0.0,
        round_trip_as_stddev: 0.0,
        total_time_ms: 0,
        responses: vec![],
    };

    let mut address_info_received = false;
    let mut responses_done = false;

    for mut sl in buffer.lines() {

        if !(address_info_received || responses_done) && sl.trim() == "" {
            //Likely windows, skip first line
            continue;
        }

        if sl.starts_with("PATTERN") {
            let mut line_parts = sl.split_whitespace();
            let pattern = line_parts.nth(1);
            if pattern.is_some() {
                data.pattern = Some(String::from(pattern.unwrap()));
            }
            continue;
        }
        if sl.starts_with("PING") {
            // Linux
            sl = sl.trim_start_matches("PING ");
            let line_parts = sl.split_once("(");
            if line_parts.is_some() {
                data.destination = String::from(line_parts.unwrap().0);
                sl = line_parts.unwrap().1;
                let mut ip_line_parts = sl.split_whitespace();

                let first = ip_line_parts.next().unwrap();

                if first.ends_with(")") {
                    data.destination_ip = String::from(first.trim_matches(|c| c == '(' || c == ')'));
                }
                else {
                    data.destination_ip = String::from(ip_line_parts.next().unwrap().trim_matches(|c| c == '(' || c == ')'));
                }

                let data_bytes = String::from(ip_line_parts.next().unwrap());
                if data_bytes.contains("(") {
                    let data_bytes_split = data_bytes.split_once("(").unwrap();
                    data.data_bytes = data_bytes_split.0.parse::<i32>().unwrap();
                }
                else {
                    data.data_bytes = data_bytes.parse::<i32>().unwrap();
                }
                //data.destination_ip = String::from(ip_line_parts.next().unwrap());
            }
            address_info_received = true;
            continue
        }
        else if sl.starts_with("Pinging") {
            println!("222222222222222222222222222222222 {}", sl);
            // Windows
            sl = sl.trim_start_matches("Pinging ");
            let mut line_parts = sl.split_whitespace();
            data.destination = String::from(line_parts.next().unwrap());
            data.destination_ip = String::from(line_parts.next().unwrap().trim_matches(|c| c == '[' || c == ']'));
            line_parts.next(); // `with`
            data.data_bytes = line_parts.next().unwrap().parse::<i32>().unwrap();
            address_info_received = true;
            continue;
        }
        if address_info_received && !responses_done {
            if sl.trim() == "" {
                responses_done = true;
                continue
            }
            // Parse the responses and add em
                //64 bytes from yyz12s08-in-x0e.1e100.net (2607:f8b0:400b:803::200e): icmp_seq=1 ttl=118 time=16.7 ms
                //64 bytes from yyz12s08-in-x0e.1e100.net (2607:f8b0:400b:803::200e): icmp_seq=2 ttl=118 time=18.3 ms
                //64 bytes from yyz12s08-in-x0e.1e100.net (2607:f8b0:400b:803::200e): icmp_seq=3 ttl=118 time=19.6 ms
            let mut response = Response {
                response_type: String::from("Reply"),
                unparsed_line: String::new(),
                timestamp: 0.0,
                bytes: 0,
                response_ip: String::new(),
                icmp_seq: 0,
                ttl: 0,
                time_ms: 0.0,
                duplicate: false,
            };
            let mut response_split = sl.split_whitespace();
            let bytes = response_split.next().unwrap();
            if bytes == "Reply" { // Windows
                response_split.next();
                response_split.next();
                let response_time = response_split.next().unwrap();
                let time_split = response_time.split_once('=').unwrap();
                response.time_ms = time_split.1.trim_end_matches("ms").parse::<f64>().unwrap();
                data.responses.push(response);
                continue;
            }
            response.bytes = bytes.parse::<i32>().unwrap();
            response_split.next();
            response_split.next();
            response_split.next();
            response.response_ip = String::from(response_split.next().unwrap().trim_matches(|c| c == '(' || c == ')' || c == ':'));
            let icmp = response_split.next();
            if icmp.is_some() && icmp.unwrap().starts_with("icmp_seq=") {
                response.icmp_seq = icmp.unwrap().split_once('=').unwrap().1.parse::<i32>().unwrap();
            }
            let ttl = response_split.next();
            if ttl.is_some() && ttl.unwrap().starts_with("ttl=") {
                response.ttl = ttl.unwrap().split_once('=').unwrap().1.parse::<i32>().unwrap();
            }
            let time = response_split.next();
            if time.is_some() && time.unwrap().starts_with("time=") {
                response.time_ms = time.unwrap().split_once('=').unwrap().1.parse::<f64>().unwrap();
            }

            if data.responses.contains(&response) {
                response.duplicate = true;
            };

            data.responses.push(response);
        }
        if address_info_received && responses_done {

            //collect stats
            if sl.starts_with("--- ") || sl.starts_with("Ping statistics for") || sl.starts_with("Approximate round trip times") {
                continue
            }
            else if sl.starts_with("rtt") {
                // rtt min/avg/max/mdev = 16.670/18.182/19.613/1.202 ms
                let line_parts = sl.split_once(" = ").unwrap();
                let mut seconds_split = line_parts.1.split("/");
                data.round_trip_as_min = seconds_split.next().unwrap().parse::<f64>().unwrap();
                data.round_trip_as_avg = seconds_split.next().unwrap().parse::<f64>().unwrap();
                data.round_trip_as_max = seconds_split.next().unwrap().parse::<f64>().unwrap();

                let stddev_split = seconds_split.next().unwrap().split_once(" ");
                data.round_trip_as_stddev = stddev_split.unwrap().0.parse::<f64>().unwrap();

            }
            else if sl.starts_with("Packets: ") {
                // Packets: Sent = 4, Received = 4, Lost = 0 (0% loss),
                sl = sl.trim_start_matches("Packets: Sent = ");
                let mut line_parts = sl.split_whitespace();
                data.packets_transmitted = line_parts.next().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
                line_parts.next();
                line_parts.next();
                data.packets_received = line_parts.next().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
                line_parts.next();
                line_parts.next();
                line_parts.next();
                data.packet_loss_percent = line_parts.next().unwrap().trim_end_matches("%").trim_start_matches("(").parse::<f64>().unwrap();

            }
            else if sl.trim().starts_with("Minimum = ") {
                // Minimum = 14ms, Maximum = 16ms, Average = 15ms
                let mut line_parts = sl.split_whitespace();
                line_parts.next();
                line_parts.next();
                data.round_trip_as_min = String::from(line_parts.next().unwrap()).trim_end_matches("ms,").parse::<f64>().unwrap();
                line_parts.next();
                line_parts.next();
                data.round_trip_as_max = String::from(line_parts.next().unwrap()).trim_end_matches("ms,").parse::<f64>().unwrap();
                line_parts.next();
                line_parts.next();
                data.round_trip_as_avg = String::from(line_parts.next().unwrap()).trim_end_matches("ms").parse::<f64>().unwrap();
                
            }
            else if sl.trim().starts_with("Packets") {
                //     Packets: Sent = 3, Received = 3, Lost = 0 (0% loss),
                let mut line_parts = sl.split_whitespace();
                line_parts.next();
                line_parts.next();
                line_parts.next();
                data.packets_transmitted = line_parts.next().unwrap().trim_end_matches(',').parse::<i32>().unwrap();
                line_parts.next();
                line_parts.next();
                data.packets_received = line_parts.next().unwrap().trim_end_matches(',').parse::<i32>().unwrap();
                line_parts.next();
                line_parts.next();
                let percent = line_parts.next().unwrap().trim_matches(|c| c == '(' || c == '%');
                data.total_time_ms = percent.parse::<i32>().unwrap();

            }
            else {
                // 3 packets transmitted, 3 received, 0% packet loss, time 2003ms
                let mut line_parts = sl.split_whitespace();
                data.packets_transmitted = line_parts.next().unwrap().parse::<i32>().unwrap();
                line_parts.next();
                line_parts.next();
                data.packets_received = line_parts.next().unwrap().parse::<i32>().unwrap();
                line_parts.next();
                data.packet_loss_percent = line_parts.next().unwrap().trim_end_matches('%').parse::<f64>().unwrap();
                line_parts.next();
                line_parts.next();
                line_parts.next();
                data.total_time_ms = line_parts.next().unwrap().trim_end_matches("ms").parse::<i32>().unwrap();
            }
        }
    }

    data
}
