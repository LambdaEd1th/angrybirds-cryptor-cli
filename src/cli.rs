use std::path::PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};

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
    Encrypt(CryptoArgs),
    /// Decrypt file
    Decrypt(CryptoArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct CryptoArgs {
    #[arg(short, long, value_name = "FILE_TYPE")]
    pub file_type: FileType,
    
    #[arg(short, long, value_name = "GAME_NAME")]
    pub game_name: GameName,
    
    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input_file: PathBuf,
    
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_file: PathBuf,
}

// 添加 Copy
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
#[clap(rename_all = "lower")]
pub enum FileType {
    Native,
    Save,
    Downloaded,
}

// 添加 Copy
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