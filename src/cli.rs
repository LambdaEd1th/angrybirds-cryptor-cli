use clap::{Args, Parser, Subcommand};
use env_logger::Builder;
use log::LevelFilter;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(
    name = "angrybirds-cryptor-cli",
    author = "ed1th",
    version,
    about = "Angry Birds file cryptor",
    long_about = "A command-line tool to encrypt and decrypt Angry Birds game data."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to a custom TOML configuration file for keys.
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// Enable verbose logging (Debug level).
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors (Error level).
    /// Conflicts with --verbose.
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,
}

impl Cli {
    /// Initializes the logging system based on CLI flags or environment variables.
    pub fn init_logger(&self) {
        let mut builder = Builder::from_default_env();

        if self.verbose {
            // -v: Show Debug information
            builder.filter_level(LevelFilter::Debug);
        } else if self.quiet {
            // -q: Only show Errors, suppress Info
            builder.filter_level(LevelFilter::Error);
        } else {
            // Default: If RUST_LOG env var is not set, default to Info
            if std::env::var("RUST_LOG").is_err() {
                builder.filter_level(LevelFilter::Info);
            }
        }

        builder.init();
    }
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum Commands {
    Encrypt(EncryptArgs),
    Decrypt(DecryptArgs),

    /// Generate a default configuration file.
    InitConfig(InitConfigArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct InitConfigArgs {
    /// Output path for the generated config file.
    #[arg(short, long, default_value = "config.toml")]
    pub output: PathBuf,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct EncryptArgs {
    #[arg(short, long, value_name = "CATEGORY", required_unless_present = "key")]
    pub category: Option<String>,

    #[arg(short, long, value_name = "GAME_NAME", required_unless_present = "key")]
    pub game: Option<String>,

    #[arg(long, value_name = "HEX_KEY")]
    pub key: Option<String>,

    #[arg(long, value_name = "HEX_IV")]
    pub iv: Option<String>,

    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct DecryptArgs {
    #[arg(short, long, value_name = "CATEGORY", required_unless_present_any = ["auto", "key"])]
    pub category: Option<String>,

    #[arg(short, long, value_name = "GAME_NAME", required_unless_present_any = ["auto", "key"])]
    pub game: Option<String>,

    #[arg(long, value_name = "HEX_KEY")]
    pub key: Option<String>,

    #[arg(long, value_name = "HEX_IV")]
    pub iv: Option<String>,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub auto: bool,

    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output: Option<PathBuf>,
}
