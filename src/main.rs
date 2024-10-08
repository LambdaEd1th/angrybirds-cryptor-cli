use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

mod crypto;
use crypto::Cryptor;

mod cli;
use cli::{Cli, Commands};

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
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
