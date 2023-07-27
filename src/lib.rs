//! # rjc
//!
//! A binary and library for parsing various formats, and commands into various formats including but not limited to JSON, YAML, TOML.

pub use self::parsers::win32;
pub use self::parsers::unix;
pub use self::parsers::darwin;
pub use self::parsers::common;
pub use self::parsers::formats;

pub mod args;
pub mod r_io_utils;

/// Command specific parsers
pub mod parsers {
    pub mod win32 {
        pub mod dir;
        pub mod assoc;
        pub mod netstat;
        pub mod tasklist;
    }
    pub mod unix {
        pub mod ls;
        pub mod chage;
        pub mod wc;
        pub mod du;
        pub mod cksum;
        pub mod env;
        pub mod file;
        pub mod ps;
        pub mod acpi;
        pub mod passwd;
        pub mod shadow;
        pub mod time;
        pub mod timedatectl;
        pub mod w;
        pub mod sysctl;
        pub mod date;
    }
    pub mod darwin { // apple osx
        pub mod airport;
    }
    pub mod common {
        pub mod lsd;
        pub mod ping;
    }
    pub mod formats {
        pub mod email_address;
        pub mod timestamp;
        pub mod version;
        pub mod semver;
    }
}
