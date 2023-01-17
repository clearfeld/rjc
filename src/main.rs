mod args;

mod parsers {
    pub mod win32 {
        pub mod dir;
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
        args::CommandParsers::Dir(_) => {
            // println!("Dir (supports /o) parse only...");
            parsers::win32::dir::parse(args.output);
        }
        args::CommandParsers::Ls(_) => {
            parsers::unix::ls::parse(args.output);
        }
    }

    // TODO: find ansi term color crate
    // println!("\x1b\u{1b}[01;34mtarget\u{1b}[0m/");
}
