use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

const HELP_TEMPLATE: &str = "{before-help}{about} by @{author-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}";

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[command(help_template = HELP_TEMPLATE)]
pub struct Cli {
    /// What mode to run the program in
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Encrypt mode
    Encrypt(CryptoArgs),

    /// Decrypt mode
    Decrypt(CryptoArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct CryptoArgs {
    /// What file type to run the program in
    #[arg(short, long, value_name = "FILE_TYPE")]
    pub file_type: String,

    /// What game file to run the program in
    #[arg(short, long, value_name = "GAME_NAME")]
    pub game_name: String,

    /// Input file
    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input_file: PathBuf,

    /// Output file
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_file: PathBuf,
}
