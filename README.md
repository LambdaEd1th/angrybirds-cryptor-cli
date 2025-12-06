# Angry Birds Cryptor CLI

A command-line utility written in Rust to encrypt and decrypt data files for various games in the *Angry Birds* series.

## Features

* **Decrypt** game data files (e.g., levels, configurations) and save files to a readable format.
* **Encrypt** modified files back into the game's format.
* **Wide Support**: Compatible with multiple Angry Birds titles and file types (Native, Save, Downloaded).
* **Cross-Platform**: Runs on Windows, Linux, and macOS.

## Supported Games

The tool supports the following game titles:

* Angry Birds Classic
* Angry Birds Rio
* Angry Birds Seasons
* Angry Birds Space
* Angry Birds Friends
* Angry Birds Star Wars
* Angry Birds Star Wars II
* Angry Birds Stella

## Installation

### From Source

You can build the project from source using Cargo. Ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

```bash
# Clone the repository (if applicable) or navigate to the source directory
cd angrybirds-cryptor-cli

# Build the release binary
cargo build --release
````

The compiled binary will be located at `target/release/angrybirds-cryptor-cli`.

## Usage

```bash
angrybirds-cryptor-cli <COMMAND> [OPTIONS]
```

### Commands

  * `encrypt`: Encrypt a file.
  * `decrypt`: Decrypt a file.
  * `help`: Print this message or the help of the given subcommand(s).

### Options

| Option | Short | Long | Description |
| :--- | :---: | :--- | :--- |
| **File Type** | `-f` | `--file-type` | The type of file to process. <br> Values: `native`, `save`, `downloaded` |
| **Game Name** | `-g` | `--game-name` | The target game. <br> Values: `classic`, `rio`, `seasons`, `space`, `friends`, `starwars`, `starwarsii`, `stella` |
| **Input** | `-i` | `--input-file` | Path to the input file. |
| **Output** | `-o` | `--output-file` | Path to the output file. (Optional) <br> If omitted, the tool automatically appends `_decrypted` or `_encrypted` to the filename. |

### Examples

#### 1\. Decrypting a Native Game File

Decrypt a level file (`example.lua`) from *Angry Birds Classic*:

```bash
angrybirds-cryptor-cli decrypt -f native -g classic -i example.lua -o example.dec.lua
```

#### 2\. Encrypting a Save File

Encrypt a modified save file back for *Angry Birds Seasons*:

```bash
angrybirds-cryptor-cli encrypt -f save -g seasons -i progress.json -o progress.dat
```

#### 3\. Quick Decrypt (Auto-naming)

Decrypt a file without specifying an output name. This will create `highscores_decrypted.lua`:

```bash
angrybirds-cryptor-cli decrypt -f save -g rio -i highscores.lua
```

## License

This project is licensed under the **GNU General Public License v3.0**.

## Disclaimer

This tool is intended for educational purposes and for modding games you legally own. The author is not affiliated with Rovio Entertainment.
