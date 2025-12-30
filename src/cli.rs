// src/cli.rs

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use strum::EnumIter;

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

    /// Enable verbose logging (sets log level to Debug).
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Encrypt file or directory
    Encrypt(EncryptArgs),
    /// Decrypt file or directory
    Decrypt(DecryptArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct EncryptArgs {
    // If --key is provided, file_type is optional.
    #[arg(short, long, value_name = "FILE_TYPE", required_unless_present = "key")]
    pub file_type: Option<FileType>,

    // If --key is provided, game_name is optional.
    #[arg(short, long, value_name = "GAME_NAME", required_unless_present = "key")]
    pub game_name: Option<GameName>,

    /// Custom Hex Key (32 bytes / 64 hex characters).
    /// Overrides game-name/file-type lookup.
    #[arg(long, value_name = "HEX_KEY")]
    pub key: Option<String>,

    /// Custom Hex IV (16 bytes / 32 hex characters).
    /// Default is all zeros if not specified.
    #[arg(long, value_name = "HEX_IV")]
    pub iv: Option<String>,

    #[arg(short, long, value_name = "INPUT_PATH")]
    pub input_file: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_PATH")]
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct DecryptArgs {
    // If --auto or --key is present, file_type is not required.
    #[arg(
        short,
        long,
        value_name = "FILE_TYPE",
        required_unless_present_any = ["auto", "key"]
    )]
    pub file_type: Option<FileType>,

    // If --auto or --key is present, game_name is not required.
    #[arg(
        short,
        long,
        value_name = "GAME_NAME",
        required_unless_present_any = ["auto", "key"]
    )]
    pub game_name: Option<GameName>,

    /// Custom Hex Key (32 bytes / 64 hex characters).
    #[arg(long, value_name = "HEX_KEY")]
    pub key: Option<String>,

    /// Custom Hex IV (16 bytes / 32 hex characters).
    /// Default is all zeros if not specified.
    #[arg(long, value_name = "HEX_IV")]
    pub iv: Option<String>,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub auto: bool,

    #[arg(short, long, value_name = "INPUT_PATH")]
    pub input_file: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_PATH")]
    pub output_file: Option<PathBuf>,
}

#[derive(ValueEnum, EnumIter, Clone, Copy, Debug, PartialEq, Eq)]
#[clap(rename_all = "lower")]
pub enum FileType {
    Native,
    Save,
    Downloaded,
}

#[derive(ValueEnum, EnumIter, Clone, Copy, Debug, PartialEq, Eq)]
#[clap(rename_all = "lower")]
pub enum GameName {
    Classic,
    Rio,
    Seasons,
    Space,
    Friends,
    Starwars,
    Starwarsii,
    Stella,
}
