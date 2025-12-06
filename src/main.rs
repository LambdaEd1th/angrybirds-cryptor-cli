mod cli;
mod crypto;

use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    #[error(transparent)]
    CryptorError(#[from] crypto::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Encrypt(args) => {
            process_file(args, |cryptor, data| Ok(cryptor.encrypt(data)))
        }
        cli::Commands::Decrypt(args) => {
            process_file(args, |cryptor, data| Ok(cryptor.decrypt(data)?))
        }
    }
}

fn process_file<F>(args: cli::CryptoArgs, processor: F) -> Result<()>
where
    F: FnOnce(&crypto::Cryptor, &[u8]) -> Result<Vec<u8>>,
{
    let cryptor = crypto::Cryptor::new(args.file_type, args.game_name)?;

    let mut input = Vec::new();
    File::open(&args.input_file)?.read_to_end(&mut input)?;

    let output = processor(&cryptor, &input)?;
    File::create(&args.output_file)?.write_all(&output)?;

    Ok(())
}
