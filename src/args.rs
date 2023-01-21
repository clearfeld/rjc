use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct RJCArgs {
    #[command(subcommand)]
    pub command_parsers: CommandParsers,

    #[clap(short, long, value_enum, default_value_t=OutputTypes::Json)]
    pub output: OutputTypes,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputTypes {
    Json,
    // Json with formating
    Pretty,
    Yaml,
    Toml,
}

#[derive(Debug, Subcommand)]
pub enum CommandParsers {
    // win32 commands
    /// `dir` command parser
    Dir(Dir),

    // unix commands
    /// `ls` command parser
    Ls(Ls),
    Wc(Wc),
}

#[derive(Debug, Args)]
pub struct Dir {
    // -w
    // wide: Option<String>,
}

#[derive(Debug, Args)]
pub struct Ls {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Wc {
    // TODO: flags
}
