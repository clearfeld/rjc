use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct RJCArgs {
    #[command(subcommand)]
    command_parsers: CommandParsers,
}

#[derive(Debug, Subcommand)]
enum CommandParsers {
    /// `dir` command parser
    Dir(Dir)
}

#[derive(Debug, Args)]
struct Dir {
    name: Option<String>,
}
