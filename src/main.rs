mod args;
mod r_io_utils;

mod parsers {
    pub mod win32 {
        pub mod dir;
        pub mod assoc;
        pub mod netstat;
        pub mod tasklist;
    }
    pub mod unix {
        pub mod ls;
        pub mod du;
        pub mod cksum;
        pub mod env;
        pub mod file;
    }
    pub mod common {
        pub mod lsd;
    }
    pub mod formats {
        pub mod email_address;
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
            parsers::win32::dir::parse(args.output);
        }
        args::CommandParsers::Assoc(_) => {
            parsers::win32::assoc::parse(args.output);
        }
        args::CommandParsers::Netstat(_) => {
            parsers::win32::netstat::parse(args.output);
        }
        args::CommandParsers::Tasklist(_) => {
            parsers::win32::tasklist::parse(args.output);
        }

        // unix

        args::CommandParsers::Ls(_) => {
            parsers::unix::ls::parse(args.output);
        }
        args::CommandParsers::Du => {
            parsers::unix::du::parse(args.output);
        }
        args::CommandParsers::Cksum => {
            parsers::unix::cksum::parse(args.output);
        }
        args::CommandParsers::Env => {
            parsers::unix::env::parse(args.output);
        }
        args::CommandParsers::File => {
            parsers::unix::file::parse(args.output);
        }

        // common
        args::CommandParsers::Lsd => {
            parsers::common::lsd::parse(args.output);
        }

        // formats
        args::CommandParsers::EmailAddress => {
            parsers::formats::email_address::parse(args.output);
        }

    }

    // TODO: find ansi term color crate
    // println!("\x1b\u{1b}[01;34mtarget\u{1b}[0m/");
}
