use anyhow::{anyhow, Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle}; // Import indicatif
use log::{debug, error, info, warn};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

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
    if input_path.is_file() {
        debug!("Processing single file: {:?}", input_path);
        let data = fs::read(input_path).context("Failed to read input file")?;

        match processor(&data) {
            Ok(processed_data) => {
                save_output(input_path, output_path, suffix, &processed_data)?;
                info!("Successfully processed: {:?}", input_path);
            }
            Err(e) => error!("Failed to process {:?}: {}", input_path, e),
        }
    } else if input_path.is_dir() {
        info!("Processing directory: {:?}", input_path);

        // Validate output directory
        if let Some(ref out_dir) = output_path {
            if !out_dir.exists() {
                fs::create_dir_all(out_dir)?;
            } else if !out_dir.is_dir() {
                anyhow::bail!("Output path must be a directory when input is a directory.");
            }
        }

        // 1. Collect all files first to determine total count for the progress bar
        info!("Scanning files...");
        let files: Vec<_> = WalkDir::new(input_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();

        let total_files = files.len() as u64;
        info!("Found {} files. Starting processing...", total_files);

        // 2. Initialize Progress Bar
        let pb = ProgressBar::new(total_files);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        let mut success_count = 0;
        let mut fail_count = 0;

        for entry in files {
            let path = entry.path();
            // Update progress bar message with current filename (truncated if too long is handled by UI usually, but keeping it simple here)
            pb.set_message(format!("{:?}", path.file_name().unwrap_or_default()));

            let data = match fs::read(path) {
                Ok(d) => d,
                Err(e) => {
                    // Use pb.suspend to print logs without breaking the bar visual
                    pb.suspend(|| error!("Could not read {:?}: {}", path, e));
                    fail_count += 1;
                    pb.inc(1);
                    continue;
                }
            };

            match processor(&data) {
                Ok(processed_data) => {
                    let target_path = if let Some(ref out_dir) = output_path {
                        let relative = path.strip_prefix(input_path).unwrap_or(path);
                        let dest = out_dir.join(relative);

                        if let Some(parent) = dest.parent() {
                            if let Err(e) = fs::create_dir_all(parent) {
                                pb.suspend(|| error!("Failed to create dir {:?}: {}", parent, e));
                                fail_count += 1;
                                pb.inc(1);
                                continue;
                            }
                        }
                        dest
                    } else {
                        generate_suffixed_path(path, suffix)
                    };

                    if let Err(e) = fs::write(&target_path, processed_data) {
                        pb.suspend(|| error!("Failed to write to {:?}: {}", target_path, e));
                        fail_count += 1;
                    } else {
                        success_count += 1;
                    }
                }
                Err(e) => {
                    pb.suspend(|| warn!("Skipping {:?}: {}", path, e));
                    fail_count += 1;
                }
            }

            pb.inc(1);
        }

        pb.finish_with_message("Done");

        // 3. Print Summary Stats
        info!(
            "Batch processing complete. Total: {}, Success: {}, Failed: {}",
            total_files, success_count, fail_count
        );
    } else {
        anyhow::bail!("Input path does not exist: {:?}", input_path);
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

    // info! might clutter single file mode, but it's acceptable.
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
