use rjc::*;

use clap::Parser;

fn main() {
    let args = args::RJCArgs::parse();
    // println!("{:#?}", args);
    // println!("{:#?}", args.command_parsers);

    match &args.command_parsers {
        // win32

        args::CommandParsers::Dir(_) => {
            r_io_utils::print_output::<win32::dir::DirData>(
                &win32::dir::parse(None),
                args.output
            );
        }
        args::CommandParsers::Assoc(_) => {
            r_io_utils::print_output::<Vec<win32::assoc::FileTypeAssociation>>(
                &win32::assoc::parse(None),
                args.output
            );
        }
        args::CommandParsers::Netstat(_) => {
            r_io_utils::print_output::<win32::netstat::ConnectionsData>(
                &win32::netstat::parse(None),
                args.output
            );
        }
        args::CommandParsers::Tasklist(_) => {
            r_io_utils::print_output::<Vec<win32::tasklist::TasklistData>>(
                &win32::tasklist::parse(None),
                args.output
            );
        }

        // unix

        args::CommandParsers::Ls(_) => {
            r_io_utils::print_output::<unix::ls::LsData>(
                &unix::ls::parse(None),
                args.output
            );
        }
        args::CommandParsers::Wc(_) => {
            r_io_utils::print_output::<unix::wc::WcData>(
                &unix::wc::parse(None),
                args.output
            );
        }
        args::CommandParsers::Du => {
            r_io_utils::print_output::<Vec<unix::du::DuData>>(
                &unix::du::parse(None),
                args.output
            );
        }
        args::CommandParsers::Cksum => {
            r_io_utils::print_output::<Vec<unix::cksum::CksumData>>(
                &unix::cksum::parse(None),
                args.output
            );
        }
        args::CommandParsers::Env => {
            r_io_utils::print_output::<Vec<unix::env::EnvData>>(
                &unix::env::parse(None),
                args.output
            );
        }
        args::CommandParsers::File => {
            r_io_utils::print_output::<Vec<unix::file::FileData>>(
                &unix::file::parse(None),
                args.output
            );
        }
        args::CommandParsers::Ps(_) => {
            r_io_utils::print_output::<unix::ps::PsData>(
                &unix::ps::parse(None),
                args.output
            );
        }
        args::CommandParsers::Chage(_) => {
            r_io_utils::print_output::<Vec<unix::chage::ChageData>>(
                &unix::chage::parse(None),
                args.output
            );
        }
        args::CommandParsers::Acpi(_) => {
            r_io_utils::print_output::<unix::acpi::AcpiData>(
                &unix::acpi::parse(None),
                args.output
            );
        }
        args::CommandParsers::Passwd(_) => {
            r_io_utils::print_output::<unix::passwd::PasswdData>(
                &unix::passwd::parse(None),
                args.output
            );
        }
        args::CommandParsers::Shadow(_) => {
            r_io_utils::print_output::<unix::shadow::ShadowData>(
                &unix::shadow::parse(None),
                args.output
            );
        }
        args::CommandParsers::Timestamp(_) => {
            r_io_utils::print_output::<unix::timestamp::TimestampData>(
                &unix::timestamp::parse(None),
                args.output
            );
        }
        args::CommandParsers::Timedatectl(_) => {
            r_io_utils::print_output::<unix::timedatectl::TimedatectlData>(
                &unix::timedatectl::parse(None),
                args.output
            );
        }
        args::CommandParsers::Time(_) => {
            r_io_utils::print_output::<unix::time::TimeData>(
                &unix::time::parse(None),
                args.output
            );
        }
        args::CommandParsers::W(_) => {
            r_io_utils::print_output::<unix::w::WData>(
                &unix::w::parse(None),
                args.output
            );
        }
        args::CommandParsers::Sysctl(_) => {
            r_io_utils::print_output::<unix::sysctl::SysctlData>(
                &unix::sysctl::parse(None),
                args.output
            );
        }

        // darwin

        args::CommandParsers::Airport(_) => {
            r_io_utils::print_output::<darwin::airport::AirportData>(
                &darwin::airport::parse(None),
                args.output
            );
        }

        // common

        args::CommandParsers::Lsd => {
            r_io_utils::print_output::<common::lsd::LsdData>(
                &common::lsd::parse(None),
                args.output
            );
        }

        // formats

        args::CommandParsers::EmailAddress => {
            r_io_utils::print_output::<Vec<formats::email_address::EmailAddressData>>(
                &formats::email_address::parse(None),
                args.output
            );
        }

    }

    // TODO: find ansi term color crate
    // println!("\x1b\u{1b}[01;34mtarget\u{1b}[0m/");
}
