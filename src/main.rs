use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use angrybirds_cryptor_cli::{cli, crypto};

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        // Handle Encryption
        cli::Commands::Encrypt(args) => {
            let cryptor = crypto::Cryptor::new(args.file_type, args.game_name)?;

            let mut input_data = Vec::new();
            File::open(&args.input_file)?.read_to_end(&mut input_data)?;

            let output_data = cryptor.encrypt(&input_data);

            save_output(
                &args.input_file,
                args.output_file,
                "_encrypted",
                &output_data,
            )?;
        }

        // Handle Decryption
        cli::Commands::Decrypt(args) => {
            let mut input_data = Vec::new();
            File::open(&args.input_file)?.read_to_end(&mut input_data)?;

            let output_data = if args.auto {
                // Auto mode: try all keys
                println!("Auto-detecting key...");
                let (data, detected_ft, detected_gn) = crypto::try_decrypt_all(&input_data)?;
                println!(
                    "Success! Detected: Game = {:?}, Type = {:?}",
                    detected_gn, detected_ft
                );
                data
            } else {
                // Manual mode: use provided arguments
                // Since clap guarantees these are present if !auto, we can safely unwrap.
                let file_type = args.file_type.unwrap();
                let game_name = args.game_name.unwrap();

                let cryptor = crypto::Cryptor::new(file_type, game_name)?;
                cryptor.decrypt(&input_data)?
            };

            save_output(
                &args.input_file,
                args.output_file,
                "_decrypted",
                &output_data,
            )?;
        }
    }

    Ok(())
}

/// Helper function to save the processed data to a file.
/// Calculates the output filename automatically if one is not provided.
fn save_output(
    input_path: &Path,
    user_output_path: Option<PathBuf>,
    suffix: &str,
    data: &[u8],
) -> Result<()> {
    let output_path = match user_output_path {
        Some(path) => path,
        None => {
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
    File::create(&output_path)?.write_all(data)?;
    Ok(())
}
