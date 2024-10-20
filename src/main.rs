use std::{
    fs::File,
    io::{Read, Write},
};

mod crypto;
use crypto::Cryptor;

mod cli;
use cli::{Cli, Commands};

use clap::Parser;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encrypt(args) => {
            let mut input_file = File::open(args.input_file)?;
            let mut input_file_buffer: Vec<u8> = Vec::new();
            input_file.read_to_end(&mut input_file_buffer)?;
            let cryptor = Cryptor::new(&input_file_buffer);
            let output_buffer = cryptor.encrypt(&args.file_type, &args.game_name);
            let mut output_file = File::create(args.output_file)?;
            output_file.write_all(&output_buffer)?;
        }
        Commands::Decrypt(args) => {
            let mut input_file = File::open(args.input_file)?;
            let mut input_file_buffer: Vec<u8> = Vec::new();
            input_file.read_to_end(&mut input_file_buffer)?;
            let cryptor = Cryptor::new(&input_file_buffer);
            let output_buffer = cryptor.decrypt(&args.file_type, &args.game_name)?;
            let mut output_file = File::create(args.output_file)?;
            output_file.write_all(&output_buffer)?;
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    // Failed hash error
    CryptorError(crypto::Error),
    IOError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::CryptorError(err) => write!(f, "CryptorError: {err}"),
            Self::IOError(err) => write!(f, "IOError: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Self::CryptorError(err) => Some(err),
            Self::IOError(err) => Some(err),
        }
    }
}

impl From<crypto::Error> for Error {
    fn from(err: crypto::Error) -> Self {
        Self::CryptorError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}
