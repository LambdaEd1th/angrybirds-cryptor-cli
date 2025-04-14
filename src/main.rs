// --- src/main.rs ---
mod cli;
mod crypto;

use clap::{Parser, ValueEnum};
use std::{
    fs::File,
    io::{Read, Write},
};

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Encrypt(args) => {
            process_file(args, |cryptor, data| Ok(cryptor.encrypt(data)))
        }
        cli::Commands::Decrypt(args) => process_file(args, |cryptor, data| cryptor.decrypt(data)),
    }
}

fn process_file<F>(args: cli::CryptoArgs, processor: F) -> Result<()>
where
    F: FnOnce(&crypto::Cryptor, &[u8]) -> crypto::Result<Vec<u8>>,
{
    let cryptor = crypto::Cryptor::new(
        &args.file_type.to_possible_value().unwrap().get_name(),
        &args.game_name.to_possible_value().unwrap().get_name(),
    );

    let mut input = Vec::new();
    File::open(&args.input_file)?.read_to_end(&mut input)?;

    let output = processor(&cryptor, &input)?;
    File::create(&args.output_file)?.write_all(&output)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Crypto error: {0}")]
    Crypto(#[from] crypto::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
