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
            process_file(
                args,
                "_encrypted",
                |cryptor, data| Ok(cryptor.encrypt(data)),
            )
        }
        cli::Commands::Decrypt(args) => process_file(args, "_decrypted", |cryptor, data| {
            Ok(cryptor.decrypt(data)?)
        }),
    }
}

fn process_file<F>(args: cli::CryptoArgs, suffix: &str, processor: F) -> Result<()>
where
    F: FnOnce(&crypto::Cryptor, &[u8]) -> Result<Vec<u8>>,
{
    let cryptor = crypto::Cryptor::new(args.file_type, args.game_name)?;

    let mut input = Vec::new();
    File::open(&args.input_file)?.read_to_end(&mut input)?;

    let output = processor(&cryptor, &input)?;

    let output_path = match args.output_file {
        Some(path) => path,
        None => {
            let input_path = &args.input_file;
            let stem = input_path.file_stem().unwrap_or_default().to_string_lossy();

            let mut new_name = format!("{}{}", stem, suffix);

            if let Some(ext) = input_path.extension() {
                new_name.push('.');
                new_name.push_str(&ext.to_string_lossy());
            }

            input_path.with_file_name(new_name)
        }
    };

    println!("Saving to: {:?}", output_path);
    File::create(&output_path)?.write_all(&output)?;

    Ok(())
}
