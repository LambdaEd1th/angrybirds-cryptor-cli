# Angry Birds Cryptor Core

Core library for the Angry Birds file cryptor. This crate provides the underlying encryption and decryption logic (AES-256-CBC) and configuration management used by the CLI tool.

## Features

- **AES-256-CBC Encryption/Decryption**: Implements standard AES-256-CBC with PKCS7 padding.
- **Configuration Management**: Handles game-specific keys and IVs via a flexible `Config` struct.
- **Auto-Detection**: `try_decrypt_all` feature to attempt decryption against all known game configurations.
- **Built-in Defaults**: Includes default keys for popular Angry Birds games (Classic, Rio, Seasons, Space, etc.).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
angrybirds-cryptor-core = { path = "core" } # Adjust path or version as needed
```

### Basic Encryption/Decryption

```rust
use angrybirds_cryptor_core::{Cryptor, Config};

fn main() -> anyhow::Result<()> {
    // 1. Initialize Config (uses defaults)
    let config = Config::default();

    // 2. Create a Cryptor for a specific game and file category
    //    (e.g., Angry Birds Classic, "highscores" or "native" files)
    let cryptor = Cryptor::new("native", "classic", &config)?;

    let data = b"some data to encrypt";

    // Encrypt
    let encrypted = cryptor.encrypt(data);

    // Decrypt
    let decrypted = cryptor.decrypt(&encrypted)?;
    assert_eq!(decrypted, data);

    Ok(())
}
```

### Auto-Detect Game

If you have an encrypted file but don't know which game it belongs to:

```rust
use angrybirds_cryptor_core::{try_decrypt_all, Config};

fn main() {
    let encrypted_data = vec![/* ... bytes ... */];
    let config = Config::default();

    match try_decrypt_all(&encrypted_data, &config) {
        Ok((decrypted_data, category, game_name)) => {
            println!("Success! Decrypted using {} - {}", game_name, category);
        },
        Err(_) => println!("Could not decrypt data with any known keys."),
    }
}
```

## Configuration

The `Config` struct essentially maps `Game Name -> Category -> Key/IV`.
You can load a custom configuration from a TOML file:

```toml
[games.my_custom_game]
native = { key = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f", iv = "00000000000000000000000000000000" }
save = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f" # Uses default IV
```

```rust
use angrybirds_cryptor_core::Config;
use std::path::Path;

let config = Config::load_or_default(Some(Path::new("config.toml")))?;
```

## License

This project is licensed under the GPL-3.0 License.
