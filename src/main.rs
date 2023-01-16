mod args;

mod parsers {
    pub mod dir;
}

use clap::Parser;

fn main() {
    let args = args::RJCArgs::parse();
    // println!("{:#?}", args);
    // println!("{:#?}", args.command_parsers);

    match &args.command_parsers {
        args::CommandParsers::Dir(_) => {
            // println!("Dir (supports /o) parse only...");
            parsers::dir::parse();
        },
        args::CommandParsers::Ls(_) => {
            println!("Ls no impl yet...");
        }
    }

    // TODO: find ansi term color crate
    // println!("\x1b\u{1b}[01;34mtarget\u{1b}[0m/");
}
