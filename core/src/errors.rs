use cipher::block_padding::UnpadError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptorError {
    #[error(
        "Decryption failed (Padding Error). This usually means the Key or IV is incorrect for this file."
    )]
    PaddingError(#[from] UnpadError),

    #[error(
        "Unsupported combination: The file category '{0}' is not available (or unknown) for the game '{1}'."
    )]
    UnsupportedCombination(String, String),

    #[error("Auto-detection failed: Unable to find a matching key.")]
    AutoDetectionFailed,

    #[error("Invalid Hex String: {0}")]
    HexError(#[from] hex::FromHexError),

    #[error("Invalid Key/IV length: Expected {expected} bytes, got {got}.")]
    InvalidLength { expected: usize, got: usize },

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML Parsing Error: {0}")]
    TomlError(#[from] toml::de::Error),
}
