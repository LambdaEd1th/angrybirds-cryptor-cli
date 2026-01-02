use anyhow::{anyhow, Context, Result};
use clap::Parser;
use log::{debug, info};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use angrybirds_cryptor_cli::{cli, config, crypto};

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli.init_logger();

    // Load config (defaults + user overrides)
    let cfg = config::Config::load_or_default(cli.config.as_deref())?;

    match cli.command {
        cli::Commands::Encrypt(cmd_args) => {
            info!("Mode: Encrypt");

            // Priority: CLI args > Config
            let cryptor = if let Some(hex_key) = cmd_args.key {
                create_custom_cryptor(&hex_key, cmd_args.iv.as_deref())?
            } else {
                debug!("Using configuration (Key & IV).");
                let category = cmd_args.category.as_deref().ok_or_else(|| {
                    anyhow!("Category argument is required when no custom key is provided.")
                })?;
                let game_name = cmd_args.game.as_deref().ok_or_else(|| {
                    anyhow!("Game name argument is required when no custom key is provided.")
                })?;

                crypto::Cryptor::new(category, game_name, &cfg)?
            };

            process_files(
                &cmd_args.input,
                cmd_args.output,
                "_encrypted",
                |data| Ok(cryptor.encrypt(data)),
            )?;
        }

        cli::Commands::Decrypt(cmd_args) => {
            info!("Mode: Decrypt");

            process_files(
                &cmd_args.input,
                cmd_args.output,
                "_decrypted",
                |data| {
                    if let Some(hex_key) = &cmd_args.key {
                        let cryptor = create_custom_cryptor(hex_key, cmd_args.iv.as_deref())?;
                        Ok(cryptor.decrypt(data)?)
                    } else if cmd_args.auto {
                        // Auto-detection now tries all configured Key+IV pairs
                        let (decrypted, ft, gn) = crypto::try_decrypt_all(data, &cfg)?;
                        info!("Auto-detected: Game='{}', Category='{}'", gn, ft);
                        Ok(decrypted)
                    } else {
                        let category = cmd_args.category.as_deref().ok_or_else(|| {
                            anyhow!("Category argument is required for manual decryption.")
                        })?;
                        let game = cmd_args.game.as_deref().ok_or_else(|| {
                            anyhow!("Game name argument is required for manual decryption.")
                        })?;

                        let cryptor = crypto::Cryptor::new(category, game, &cfg)?;
                        Ok(cryptor.decrypt(data)?)
                    }
                },
            )?;
        }

        cli::Commands::InitConfig(cmd_args) => {
            info!("Generating default configuration...");

            let default_config = config::Config::default();

            let toml_string = toml::to_string_pretty(&default_config)
                .context("Failed to serialize default configuration")?;

            let path = cmd_args.output;
            fs::write(&path, toml_string)
                .with_context(|| format!("Failed to write config file to {:?}", path))?;

            info!("Successfully created default config at {:?}", path);
        }
    }

    Ok(())
}

fn create_custom_cryptor(hex_key: &str, hex_iv: Option<&str>) -> Result<crypto::Cryptor> {
    let key_bytes = hex::decode(hex_key).context("Failed to decode hex key")?;

    if key_bytes.len() != 32 {
        return Err(anyhow!(
            "Key must be 32 bytes (64 hex chars), got {} bytes",
            key_bytes.len()
        ));
    }

    let key_array: [u8; 32] = key_bytes.try_into().map_err(|v: Vec<u8>| {
        anyhow!(
            "Internal error: Key vector conversion failed (len={})",
            v.len()
        )
    })?;

    let iv_array = if let Some(iv_str) = hex_iv {
        let iv_bytes = hex::decode(iv_str).context("Failed to decode hex IV")?;

        if iv_bytes.len() != 16 {
            return Err(anyhow!(
                "IV must be 16 bytes (32 hex chars), got {} bytes",
                iv_bytes.len()
            ));
        }

        Some(iv_bytes.try_into().map_err(|v: Vec<u8>| {
            anyhow!(
                "Internal error: IV vector conversion failed (len={})",
                v.len()
            )
        })?)
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
    F: Fn(&[u8]) -> Result<Vec<u8>>,
{
    if input_path.is_dir() {
        return Err(anyhow!("Directory processing disabled"));
    }
    if !input_path.exists() {
        return Err(anyhow!("Input file not found"));
    }
    let data = fs::read(input_path)?;
    let res = processor(&data)?;
    save_output(input_path, output_path, suffix, &res)
}

fn save_output(input: &Path, output: Option<PathBuf>, suffix: &str, data: &[u8]) -> Result<()> {
    let out = output.unwrap_or_else(|| generate_suffixed_path(input, suffix));
    File::create(out)?.write_all(data)?;
    Ok(())
}

fn generate_suffixed_path(path: &Path, suffix: &str) -> PathBuf {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let ext = path
        .extension()
        .map(|e| e.to_string_lossy())
        .unwrap_or_default();
    let new_name = if ext.is_empty() {
        format!("{}{}", stem, suffix)
    } else {
        format!("{}{}.{}", stem, suffix, ext)
    };
    path.with_file_name(new_name)
}
