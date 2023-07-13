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
    /// `assoc` command parser
    Assoc(Assoc),
    /// "`netstat` -an" command parser
    Netstat(Netstat),
    /// `tasklist` command parser
    Tasklist(Tasklist),

    // unix commands
    /// `ls` command parser
    Ls(Ls),
    /// `wc` command parser
    Wc(Wc),
    /// `du` command parser
    Du,
    /// `cksum` command parser
    Cksum,
    /// `env` command parser
    Env,
    /// `file` command parser
    File,
    /// `chage` command parser
    Chage(Chage),
    /// `acpi` command parser
    Acpi(Acpi),
    // `passwd
    Passwd(Passwd),
    /// `shadow` command parser
    Shadow(Shadow),
    /// `w` command parser
    W(W),

    // osx commands
    /// `airport` command parser
    Airport(Airport),

    // common
    Lsd,

    // formats
    /// `email address` format parser
    EmailAddress,

    /// `ps` command parser
    Ps(Ps),
}

// win32

#[derive(Debug, Args)]
pub struct Dir {
    // -w
    // wide: Option<String>,
}

#[derive(Debug, Args)]
pub struct Assoc {}

#[derive(Debug, Args)]
pub struct Netstat {}

#[derive(Debug, Args)]
pub struct Tasklist {}

// unix

#[derive(Debug, Args)]
pub struct Ls {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Wc {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Ps {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Chage {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Shadow {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Acpi {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct W {
    // TODO: flags
}

// osx

#[derive(Debug, Args)]
pub struct Airport {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Passwd {
    // TODO: flags
}
