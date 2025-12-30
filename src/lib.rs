// src/lib.rs

// Expose the internal modules
pub mod cli;
pub mod constants;
pub mod crypto;

// Re-export commonly used types
pub use cli::{Cli, Commands, FileType, GameName};
pub use crypto::Cryptor;
// We can also re-export constants if needed, but usually accessing them via the module is fine.