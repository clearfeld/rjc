# rjc

```bash
dir | rjc dir
```

```json
{"meta":{"drive":"F","serial":"38EC-3395","directory":"F:\\Dev\\rjc","files":3,"directories":4},"resources":[{"date":"01/15/2023","time":"04:14 PM","is_dir":true,"size":null,"name":"."},{"date":"01/15/2023","time":"04:14 PM","is_dir":true,"size":null,"name":".."},{"date":"01/14/2023","time":"04:25 PM","is_dir":false,"size":8,"name":".gitignore"},{"date":"01/15/2023","time":"10:38 PM","is_dir":false,"size":11117,"name":"Cargo.lock"},{"date":"01/20/2023","time":"12:53 AM","is_dir":false,"size":437,"name":"Cargo.toml"},{"date":"01/21/2023","time":"02:18 PM","is_dir":true,"size":null,"name":"src"},{"date":"01/20/2023","time":"12:53 AM","is_dir":true,"size":null,"name":"target"}]}
```

## Installation

### Cargo Install (MacOS, Linux, Windows)

<p>
    <a href="https://crates.io/crates/rjc">
        <img src="https://img.shields.io/crates/v/rjc.svg" />
    </a>
    <a href="https://docs.rs/rjc">
        <img src="https://img.shields.io/badge/docs.rs-rjc-green" />
    </a>
</p>

```bash
cargo install rjc
```

### Binaries

For precompiled binaries, check the [releases](https://github.com/clearfeld/rjc/releases) in this repo.

## Library

`rjc` can also be used as a library.

```rust
use rjc::win32::dir::{DirData, parse};
use std::process::Command;

fn main() {
    let output = Command::new("cmd").args(["/C", "dir"]).output().expect("Failed to execute process.");

    let dir_data: DirData = parse(Some(String::from_utf8_lossy(&output.stdout).to_string()));

    println!("{}", dir_data.meta.drive);
}
```

## Parsers

### Win32

| Commands  | Documentation                                            |
| --------- | -------------------------------------------------------- |
| assoc     | [details](https://rjc.vercel.app/parsers/win32/assoc)    |
| dir       | [details](https://rjc.vercel.app/parsers/win32/dir)      |
| netstat   | [details](https://rjc.vercel.app/parsers/win32/netstat)  |
| tasklist  | [details](https://rjc.vercel.app/parsers/win32/tasklist) |

### Unix

| Commands     | Documentation                                                |
| ------------ | ------------------------------------------------------------ |
| acpi         | [details](https://rjc.vercel.app/parsers/unix/acpi)          |
| arp          | [details](https://rjc.vercel.app/parsers/unix/arp)           |
| chage        | [details](https://rjc.vercel.app/parsers/unix/chage)         |
| cksum        | [details](https://rjc.vercel.app/parsers/unix/cksum)         |
| date         | [details](https://rjc.vercel.app/parsers/unix/date)          |
| du           | [details](https://rjc.vercel.app/parsers/unix/du)            |
| env          | [details](https://rjc.vercel.app/parsers/unix/env)           |
| file         | [details](https://rjc.vercel.app/parsers/unix/file)          |
| ls           | [details](https://rjc.vercel.app/parsers/unix/ls)            |
| passwd       | [details](https://rjc.vercel.app/parsers/unix/passwd)        |
| time         | [details](https://rjc.vercel.app/parsers/unix/time)          |
| timedatectl  | [details](https://rjc.vercel.app/parsers/unix/timedatectl)   |
| shadow       | [details](https://rjc.vercel.app/parsers/unix/shadow)        |
| sysctl       | [details](https://rjc.vercel.app/parsers/unix/sysctl)        |
| w            | [details](https://rjc.vercel.app/parsers/unix/w)             |
| wc           | [details](https://rjc.vercel.app/parsers/unix/wc)            |

### Darwin

| Commands    | Documentation                                            |
| ----------- | -------------------------------------------------------- |
| airport     | [details](https://rjc.vercel.app/parsers/darwin/airport) |

### External

| Commands                                     | Documentation                                         |
| -------------------------------------------- | ----------------------------------------------------- |
| [lsd](https://github.com/Peltoche/lsd)       | [details](https://rjc.vercel.app/parsers/common/lsd)  |
| ping                                         | [details](https://rjc.vercel.app/parsers/common/ping) |

### Formats

| Commands          | Documentation                                               |
| ----------------- | ----------------------------------------------------------- |
| email-address     | [details](https://rjc.vercel.app/parsers/formats/email)     |
| semver            | [details](https://rjc.vercel.app/parsers/formats/semver)    |
| timestamp         | [details](https://rjc.vercel.app/parsers/formats/timestamp) |
| version           | [details](https://rjc.vercel.app/parsers/formats/version)   |
