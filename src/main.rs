use rjc::*;

use clap::Parser;

fn main() {
    let args = args::RJCArgs::parse();
    // println!("{:#?}", args);
    // println!("{:#?}", args.command_parsers);

    match &args.command_parsers {
        // win32

        args::CommandParsers::Dir(_) => {
            parsers::win32::dir::print(parsers::win32::dir::parse(None), args.output);
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
        args::CommandParsers::Wc(_) => {
            parsers::unix::wc::parse(args.output);
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
        args::CommandParsers::Ps(_) => {
            parsers::unix::ps::parse(args.output);
        }
        args::CommandParsers::Chage(_) => {
            parsers::unix::chage::parse(args.output);
        }
        args::CommandParsers::Acpi(_) => {
            parsers::unix::acpi::parse(args.output);
        }
        args::CommandParsers::Passwd(_) => {
            parsers::unix::passwd::parse(args.output);
        }
        args::CommandParsers::Shadow(_) => {
            parsers::unix::shadow::parse(args.output);
        }
        args::CommandParsers::W(_) => {
            parsers::unix::w::parse(args.output);
        }

        // darwin

        args::CommandParsers::Airport(_) => {
            parsers::darwin::airport::parse(args.output);
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
