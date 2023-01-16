use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct RJCArgs {
    #[command(subcommand)]
    pub command_parsers: CommandParsers,
}

#[derive(Debug, Subcommand)]
pub enum CommandParsers {
    // win32 commands
    /// `dir` command parser
    Dir(Dir),

    // unix commands
    /// `ls` command parser
    Ls(Ls)
}

#[derive(Debug, Args)]
pub struct Dir {
    // name: Option<String>,
}

#[derive(Debug, Args)]
pub struct Ls {

}

