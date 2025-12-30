use crate::cli::{FileType, GameName};
use aes::cipher::block_padding::UnpadError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptorError {
    /// Wrapper for AES decryption padding errors.
    /// This is the most common error when the wrong key is used.
    #[error("Decryption failed (Padding Error). This usually means the Key or IV is incorrect for this file.")]
    PaddingError(#[from] UnpadError),

    /// Error when the user requests a game/file-type combination that has no known key.
    #[error("Unsupported combination: The file type '{0:?}' is not available (or unknown) for the game '{1:?}'.")]
    UnsupportedCombination(FileType, GameName),

    /// Error when auto-detection tries all keys but fails to decrypt.
    #[error("Auto-detection failed: Unable to find a matching key. The file might be corrupted, or it belongs to an unsupported game version.")]
    AutoDetectionFailed,

    /// Error for invalid hex strings provided via CLI arguments.
    #[error("Invalid Hex String: {0}")]
    HexError(#[from] hex::FromHexError),

    /// Error when Key or IV length is incorrect.
    #[error("Invalid Key/IV length: Expected {expected} bytes, got {got}.")]
    InvalidLength { expected: usize, got: usize },
}
