// Expose the internal modules so they can be used by the binary or external crates.
pub mod cli;
pub mod crypto;

// Re-export commonly used types for convenient access.
// This allows users to import them directly from the crate root (e.g., use angrybirds_cryptor_cli::Cryptor).
pub use crypto::Cryptor;
pub use cli::{Cli, Commands, FileType, GameName};