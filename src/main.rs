mod args;

use args::RJCArgs;
use clap::Parser;

fn main() {
    let args = RJCArgs::parse();
    println!("{:#?}", args);
}
