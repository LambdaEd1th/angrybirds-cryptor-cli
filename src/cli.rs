// --- src/cli.rs ---
use std::path::PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};

const HELP_TEMPLATE: &str = "{before-help}{about} by @{author-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}";

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[command(help_template = HELP_TEMPLATE)]
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

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
#[clap(rename_all = "lower")]
pub enum FileType {
    Native,
    Save,
    Downloaded,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
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