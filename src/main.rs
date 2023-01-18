mod args;
mod r_io_utils;

mod parsers {
    pub mod win32 {
        pub mod dir;
        pub mod assoc;
        pub mod netstat;
    }
    pub mod unix {
        pub mod ls;
    }
}

use clap::Parser;

fn main() {
    let args = args::RJCArgs::parse();
    // println!("{:#?}", args);
    // println!("{:#?}", args.command_parsers);

    match &args.command_parsers {
        // win32

        args::CommandParsers::Dir(_) => {
            // println!("Dir (supports /o) parse only...");
            parsers::win32::dir::parse(args.output);
        }
        args::CommandParsers::Assoc(_) => {
            // println!("Dir (supports /o) parse only...");
            parsers::win32::assoc::parse(args.output);
        }
        args::CommandParsers::Netstat(_) => {
            // println!("Dir (supports /o) parse only...");
            parsers::win32::netstat::parse(args.output);
        }

        // unix

        args::CommandParsers::Ls(_) => {
            parsers::unix::ls::parse(args.output);
        }
    }

    // TODO: find ansi term color crate
    // println!("\x1b\u{1b}[01;34mtarget\u{1b}[0m/");
}
