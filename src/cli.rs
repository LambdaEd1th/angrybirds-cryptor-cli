// src/cli.rs

use clap::{Args, Parser, Subcommand, ValueEnum};
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
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Encrypt file
    Encrypt(EncryptArgs),
    /// Decrypt file
    Decrypt(DecryptArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct EncryptArgs {
    #[arg(short, long, value_name = "FILE_TYPE")]
    pub file_type: FileType,

    #[arg(short, long, value_name = "GAME_NAME")]
    pub game_name: GameName,

    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input_file: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct DecryptArgs {
    // If --auto is present, file_type is not required.
    #[arg(
        short,
        long,
        value_name = "FILE_TYPE",
        required_unless_present = "auto"
    )]
    pub file_type: Option<FileType>,

    // If --auto is present, game_name is not required.
    #[arg(
        short,
        long,
        value_name = "GAME_NAME",
        required_unless_present = "auto"
    )]
    pub game_name: Option<GameName>,

    // Flag to enable automatic key detection.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub auto: bool,

    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input_file: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_file: Option<PathBuf>,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
#[clap(rename_all = "lower")]
pub enum FileType {
    Native,
    Save,
    Downloaded,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
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
