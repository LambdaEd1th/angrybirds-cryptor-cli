// Expose the internal modules
pub mod cli;
pub mod constants;
pub mod crypto;
pub mod errors;

// Re-export commonly used types
pub use cli::{Cli, Commands, FileType, GameName};
pub use crypto::Cryptor;
pub use errors::CryptorError; // 2. Re-export the error type