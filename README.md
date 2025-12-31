# Angry Birds Cryptor CLI

**Angry Birds Cryptor CLI** is a robust, cross-platform command-line tool written in **Rust**. It allows users to decrypt and encrypt data files (such as levels, save data, and high scores) used in various *Angry Birds* games.

This tool is designed for modders, researchers, and enthusiasts who wish to analyze or modify game files legally.

## 🚀 Key Features

* **AES-256-CBC Support**: Implements the standard encryption algorithm used by the game engine.
* **Auto-Detection**: The `decrypt` command can automatically brute-force through known keys to identify the correct game and file category.
* **Multiple File Categories**: Supports `native` (game data), `save` (progress files), and `downloaded` (DLC) formats.
* **Custom Configuration**: Users can provide their own keys via a `config.toml` file or CLI arguments.
* **Cross-Platform**: Compiles for **Windows**, **Linux**, and **macOS** (both Intel and Apple Silicon).

## 🎮 Supported Games

The tool includes built-in keys for the following titles:

* **Angry Birds Classic**
* **Angry Birds Rio**
* **Angry Birds Seasons**
* **Angry Birds Space**
* **Angry Birds Friends**
* **Angry Birds Star Wars**
* **Angry Birds Star Wars II**
* **Angry Birds Stella**

## 📦 Installation

### Option 1: Download Binary

Check the [Releases](https://www.google.com/search?q=https://github.com/LambdaEd1th/angrybirds-cryptor-cli/releases) page for pre-compiled binaries for your operating system.

### Option 2: Build from Source

Ensure you have the [Rust toolchain](https://www.rust-lang.org/) installed (Cargo).

```bash
# Clone the repository
git clone https://github.com/LambdaEd1th/angrybirds-cryptor-cli.git
cd angrybirds-cryptor-cli

# Build for release
cargo build --release

```

The binary will be available at `./target/release/angrybirds-cryptor-cli`.

## 🛠 Usage

```bash
angrybirds-cryptor-cli <COMMAND> [OPTIONS]

```

### Commands

* `encrypt`: Encrypt a raw file back into the game format.
* `decrypt`: Decrypt an encrypted game file.
* `init-config`: Generate a default `config.toml` file for customization.
* `help`: Display help information.

### 🔓 Decrypting Files

**Method 1: Automatic Detection (Recommended)**
If you don't know the specific game or file category, use the `--auto` flag. The tool will try all known key combinations.

```bash
angrybirds-cryptor-cli decrypt --input highscores.lua --auto

```

**Method 2: Manual Specification**
Manually specify the game and file category.

```bash
angrybirds-cryptor-cli decrypt \
  --game classic \
  --category native \
  --input levels.lua \
  --output levels.dec.lua

```

**Method 3: Custom Keys**
Use a specific Hex Key and IV (Initialization Vector).

```bash
angrybirds-cryptor-cli decrypt \
  --key "55534361505170413454534e56784d49317639534b39554330795a75416e6232" \
  --iv "00000000000000000000000000000000" \
  --input data.enc

```

### 🔒 Encrypting Files

To encrypt a modified file back to the game format:

```bash
angrybirds-cryptor-cli encrypt \
  --game seasons \
  --category save \
  --input settings.lua \
  --output settings.dec.lua

```

### ⚙️ Configuration

You can override the built-in keys or add new ones by using a configuration file.

1. Generate a template config:
```bash
angrybirds-cryptor-cli init-config --output my_config.toml

```


2. Edit `my_config.toml` to add your custom keys.
3. Run the tool with the `--config` flag:
```bash
angrybirds-cryptor-cli decrypt --config my_config.toml ...

```



## 📋 Options Reference

| Option | Short | Description |
| --- | --- | --- |
| `--game` | `-g` | Target game (e.g., `classic`, `rio`, `space`). |
| `--category` | `-c` | File category (`native`, `save`, `downloaded`). |
| `--input` | `-i` | Path to the source file. |
| `--output` | `-o` | (Optional) Path to the destination file. |
| `--key` |  | 32-byte Key in Hex string format. |
| `--iv` |  | 16-byte IV in Hex string format. |
| `--auto` | `-a` | (Decrypt only) Attempt to auto-detect the key. |
| `--verbose` | `-v` | Enable debug logging. |
| `--quiet` | `-q` | Suppress non-error output. |

## ⚖️ License

This project is open-source software licensed under the **GNU General Public License v3.0**.

## ⚠️ Disclaimer

This tool is provided for educational and interoperability purposes only. It is not affiliated with or endorsed by Rovio Entertainment. Please respect the intellectual property rights of the game developers.