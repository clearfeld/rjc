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
    /// `passwd` command parser
    Passwd(Passwd),
    /// `shadow` command parser
    Shadow(Shadow),
    /// `timedatectl` command parser
    Timedatectl(Timedatectl),
    /// `time` command parser
    Time(Time),
    /// `w` command parser
    W(W),
    /// `sysctl` command parser
    Sysctl(Sysctl),
    /// `date` command parser
    Date(Date),
    /// `systemctl` command parser
    Systemctl(Systemctl),
    /// `systemctl_lj` command parser
    SystemctlLJ(SystemctlLJ),
    /// `systemctl_ls` command parser
    SystemctlLS(SystemctlLS),
    /// `systemctl_luf` command parser
    SystemctlLUF(SystemctlLUF),

    // osx commands
    /// `airport` command parser
    Airport(Airport),

    // common
    /// `lsd` command parser
    Lsd,

    // formats
    /// `email address` format parser
    EmailAddress,
    /// `timestamp` command parser
    Timestamp(Timestamp),
    /// `version` command parser
    Version(Version),
    /// `semver` command parser
    Semver(Semver),

    /// `ps` command parser
    Ps(Ps),
}

// win32

#[derive(Debug, Args)]
pub struct Dir {
    // /// "wide" command flag
    // #[clap(short, long)]
    // wide: bool,
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
pub struct Timedatectl {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Time {
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

#[derive(Debug, Args)]
pub struct Timestamp {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Version {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Sysctl {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Date {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Systemctl {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct SystemctlLJ {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct SystemctlLS {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct SystemctlLUF {
    // TODO: flags
}

// osx

#[derive(Debug, Args)]
pub struct Airport {
    // TODO: flags

    /// "s" flag
    #[clap(short, long)]
    s: bool,
}

#[derive(Debug, Args)]
pub struct Passwd {
    // TODO: flags
}

#[derive(Debug, Args)]
pub struct Semver {
    // TODO: flags
}
