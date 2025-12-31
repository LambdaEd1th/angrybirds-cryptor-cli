use clap::{Args, Parser, Subcommand};
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
    #[arg(short, long, value_name = "FILE_TYPE", required_unless_present = "key")]
    pub file_type: Option<String>,

    #[arg(short, long, value_name = "GAME_NAME", required_unless_present = "key")]
    pub game_name: Option<String>,

    #[arg(long, value_name = "HEX_KEY")]
    pub key: Option<String>,

    #[arg(long, value_name = "HEX_IV")]
    pub iv: Option<String>,

    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input_file: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct DecryptArgs {
    #[arg(short, long, value_name = "FILE_TYPE", required_unless_present_any = ["auto", "key"])]
    pub file_type: Option<String>,

    #[arg(short, long, value_name = "GAME_NAME", required_unless_present_any = ["auto", "key"])]
    pub game_name: Option<String>,

    #[arg(long, value_name = "HEX_KEY")]
    pub key: Option<String>,

    #[arg(long, value_name = "HEX_IV")]
    pub iv: Option<String>,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub auto: bool,

    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input_file: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_file: Option<PathBuf>,
}
