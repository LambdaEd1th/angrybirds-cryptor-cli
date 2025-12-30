use anyhow::{anyhow, Context, Result};
use clap::Parser;
use log::{debug, error, info};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

// Import constants
use angrybirds_cryptor_cli::{cli, constants, crypto};

fn main() -> Result<()> {
    // 1. Initialize Logger
    let cli = cli::Cli::parse();
    let default_log_level = if cli.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(default_log_level))
        .init();

    // 2. Process Commands
    match cli.command {
        cli::Commands::Encrypt(args) => {
            info!("Mode: Encrypt");

            let cryptor = if let Some(hex_key) = args.key {
                debug!("Using custom hex key.");
                create_custom_cryptor(&hex_key, args.iv.as_deref())?
            } else {
                debug!("Using built-in key lookup.");
                crypto::Cryptor::new(args.file_type.unwrap(), args.game_name.unwrap())?
            };

            process_files(
                &args.input_file,
                args.output_file,
                constants::SUFFIX_ENCRYPTED,
                |data| Ok(cryptor.encrypt(data)),
            )?;
        }

        cli::Commands::Decrypt(args) => {
            info!("Mode: Decrypt");

            process_files(
                &args.input_file,
                args.output_file,
                constants::SUFFIX_DECRYPTED,
                |data| {
                    if let Some(hex_key) = &args.key {
                        debug!("Decrypting with custom hex key.");
                        let cryptor = create_custom_cryptor(hex_key, args.iv.as_deref())?;
                        Ok(cryptor.decrypt(data)?)
                    } else if args.auto {
                        let (decrypted, _, _) = crypto::try_decrypt_all(data)?;
                        Ok(decrypted)
                    } else {
                        let cryptor =
                            crypto::Cryptor::new(args.file_type.unwrap(), args.game_name.unwrap())?;
                        Ok(cryptor.decrypt(data)?)
                    }
                },
            )?;
        }
    }

    Ok(())
}

fn create_custom_cryptor(hex_key: &str, hex_iv: Option<&str>) -> Result<crypto::Cryptor> {
    let key_bytes = hex::decode(hex_key).context("Failed to decode hex key")?;
    if key_bytes.len() != 32 {
        return Err(anyhow!(
            "Key must be 32 bytes (64 hex characters), got {}",
            key_bytes.len()
        ));
    }
    let key_array: [u8; 32] = key_bytes.try_into().expect("Length checked above");

    let iv_array = if let Some(iv_str) = hex_iv {
        let iv_bytes = hex::decode(iv_str).context("Failed to decode hex IV")?;
        if iv_bytes.len() != 16 {
            return Err(anyhow!(
                "IV must be 16 bytes (32 hex characters), got {}",
                iv_bytes.len()
            ));
        }
        Some(iv_bytes.try_into().expect("Length checked above"))
    } else {
        None
    };

    Ok(crypto::Cryptor::new_custom(key_array, iv_array))
}

fn process_files<F>(
    input_path: &Path,
    output_path: Option<PathBuf>,
    suffix: &str,
    processor: F,
) -> Result<()>
where
    F: Fn(&[u8]) -> Result<Vec<u8>> + Copy,
{
    // Explicitly reject directories
    if input_path.is_dir() {
        return Err(anyhow!(
            "Directory processing is disabled. Please specify a single file path: {:?}",
            input_path
        ));
    }

    if !input_path.exists() {
        return Err(anyhow!("Input file does not exist: {:?}", input_path));
    }

    // Process single file
    debug!("Processing single file: {:?}", input_path);
    let data = fs::read(input_path).context("Failed to read input file")?;

    match processor(&data) {
        Ok(processed_data) => {
            save_output(input_path, output_path, suffix, &processed_data)?;
            info!("Successfully processed: {:?}", input_path);
        }
        Err(e) => {
            error!("Failed to process {:?}: {}", input_path, e);
            // We propagate the error up so the CLI exits with non-zero status code on failure
            return Err(anyhow!("Processing failed for file: {:?}", input_path));
        }
    }

    Ok(())
}

fn save_output(
    input_path: &Path,
    user_output_path: Option<PathBuf>,
    suffix: &str,
    data: &[u8],
) -> Result<()> {
    let output_path = match user_output_path {
        Some(path) => path,
        None => generate_suffixed_path(input_path, suffix),
    };

    info!("Saving output to: {:?}", output_path);
    File::create(&output_path)?.write_all(data)?;
    Ok(())
}

fn generate_suffixed_path(path: &Path, suffix: &str) -> PathBuf {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let mut new_name = format!("{}{}", stem, suffix);

    if let Some(ext) = path.extension() {
        new_name.push('.');
        new_name.push_str(&ext.to_string_lossy());
    }

    path.with_file_name(new_name)
}
